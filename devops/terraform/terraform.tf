terraform {

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }

  backend "s3" {
    # bucket name
    bucket = "climate-news-terraform-state"
    key    = "global/s3/terraform.tfstate"
    region = "us-east-2"

    # DynamoDB table name
    dynamodb_table = "climate-news-terraform-state-lock-table"
    encrypt        = true
  }

}

provider "aws" {
  region = var.aws_region
}



