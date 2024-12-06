import boto3
from typing import List
from botocore.exceptions import ClientError

def list_bedrock_models(region_name: str = 'us-east-1') -> List[str]:
    """
    List available models in Amazon Bedrock
    """
    try:
        # Use bedrock instead of bedrock-runtime
        client = boto3.client('bedrock', region_name=region_name)
        response = client.list_foundation_models()
        
        # Extract model IDs from response
        models = [model['modelId'] for model in response['modelSummaries']]
        return models

    except ClientError as e:
        print(f"Error accessing Bedrock: {e}")
        return []

if __name__ == '__main__':
    models = list_bedrock_models()
    if models:
        print("Available Bedrock models:")
        for model in models:
            print(f"- {model}")
    else:
        print("No models found or error occurred")