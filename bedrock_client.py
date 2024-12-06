# bedrock_client.py
import boto3

class BedrockClient:
    _instance = None
    
    @classmethod
    def get_client(cls):
        if cls._instance is None:
            cls._instance = boto3.client('bedrock-runtime', region_name='us-east-1')
        return cls._instance