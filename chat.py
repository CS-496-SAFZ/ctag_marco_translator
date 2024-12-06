import subprocess
import os
from pathlib import Path
from typing import List, Dict, Tuple
from langchain.llms.bedrock import Bedrock
from bedrock_client import BedrockClient
import logging
import time
from functools import wraps
from botocore.exceptions import ClientError
import argparse

def retry_with_backoff(max_retries=5, initial_delay=1):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            delay = initial_delay
            for retry in range(max_retries):
                try:
                    return func(*args, **kwargs)
                except ClientError as e:
                    error_code = e.response.get('Error', {}).get('Code', '')
                    if error_code != 'ThrottlingException':
                        raise
                    if retry == max_retries - 1:
                        raise
                    
                    sleep_time = delay * (2 ** retry)
                    logging.info(f"Throttled. Retrying in {sleep_time} seconds...")
                    time.sleep(sleep_time)
            return None
        return wrapper
    return decorator

class CRustConverter:
    def __init__(self, model_id: str = 'anthropic.claude-v2'):
        self.boto3_bedrock = BedrockClient.get_client()
        self.llm = Bedrock(
            model_id=model_id,
            client=self.boto3_bedrock,
            # Remove max_tokens parameter
        )
        self.setup_logging()

    def setup_logging(self):
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(levelname)s - %(message)s'
        )

    def extract_macros(self, file_path: str) -> List[Dict]:
        """Extract macros using ctags"""
        try:
            cmd = ['ctags', '-x', '--c-kinds=d', file_path]
            output = subprocess.check_output(cmd).decode()
            
            macros = []
            for line in output.splitlines():
                parts = line.split(None, 5)
                if len(parts) >= 6:
                    macro = {
                        'name': parts[0],
                        'line': int(parts[2]),
                        'definition': parts[5]
                    }
                    macros.append(macro)
            return macros
        except subprocess.CalledProcessError as e:
            logging.error(f"Failed to run ctags: {e}")
            return []

    def chunk_code(self, code: str, max_size: int) -> List[str]:
        """Split code into chunks"""
        lines = code.splitlines()
        if len(lines) <= max_size:
            return [code]
        
        chunks = []
        for i in range(0, len(lines), max_size):
            chunk = '\n'.join(lines[i:i + max_size])
            chunks.append(chunk)
        return chunks
    
    @retry_with_backoff(max_retries=5, initial_delay=1)
    def convert_code(self, c_code: str, macros: List[Dict], max_chunk_size: int = 1000) -> str:
        """Convert C code to Rust with macro context"""
        chunks = self.chunk_code(c_code, max_chunk_size)
        rust_chunks = []

        for chunk in chunks:
            # Create prompt with macro context
            prompt = f"""
            Macro definitions from this C file:
            {self.format_macros(macros)}
            
            Convert this C code to idiomatic Rust, considering the macro definitions above:
            ```c
            {chunk}
            ```
            """
            
            try:
                response = self.llm.predict(prompt)
                rust_chunks.append(response)
            except Exception as e:
                if "context window" in str(e).lower():
                    # Retry with smaller chunk
                    return self.convert_code(chunk, macros, max_chunk_size // 2)
                else:
                    logging.error(f"LLM error: {e}")
                    raise

        return '\n'.join(rust_chunks)
    
    def format_macros(self, macros: List[Dict]) -> str:
        """Format macros for prompt context"""
        return '\n'.join([
            f"#define {m['name']} {m['definition']}"
            for m in macros
        ])

    def process_file(self, file_path: str, input_dir: str, output_dir: str):
        """Process a single C file preserving directory structure"""
        try:
            # Get relative path from input dir
            rel_path = os.path.relpath(file_path, input_dir)
            # Create output path with same structure
            out_path = Path(output_dir) / Path(rel_path).with_suffix('.rs')
            
            # Create parent directories
            out_path.parent.mkdir(parents=True, exist_ok=True)
            
            # Extract macros
            macros = self.extract_macros(file_path)
            
            # Read C code
            with open(file_path) as f:
                c_code = f.read()

            # Convert to Rust
            rust_code = self.convert_code(c_code, macros)

            # Save output
            with open(out_path, 'w') as f:
                f.write(rust_code)

            logging.info(f"Converted {file_path} -> {out_path}")

        except Exception as e:
            logging.error(f"Failed to process {file_path}: {e}")

def main():
    parser = argparse.ArgumentParser(
        description='Convert C code to Rust using LLM'
    )
    parser.add_argument(
        '-i', '--input',
        required=True,
        help='Input directory containing C files'
    )
    parser.add_argument(
        '-o', '--output',
        required=True,
        help='Output directory for Rust files'
    )
    
    args = parser.parse_args()
    converter = CRustConverter()
    
    # Process all .c files preserving structure
    for root, _, files in os.walk(args.input):
        for file in files:
            if file.endswith('.c'):
                file_path = os.path.join(root, file)
                converter.process_file(file_path, args.input, args.output)

if __name__ == '__main__':
    main()