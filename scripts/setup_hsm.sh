#!/bin/bash
# HSM Configuration Script

AWS_REGION=${1:-us-west-2}
KEY_ARN=$(aws kms create-key --region $AWS_REGION --query 'KeyMetadata.Arn' --output text)

echo "Created KMS Key ARN: $KEY_ARN"
echo "key_arn = \"$KEY_ARN\"" > config/hsm.toml
