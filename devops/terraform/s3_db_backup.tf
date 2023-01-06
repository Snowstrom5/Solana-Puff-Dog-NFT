# S3 bucket used for database backups
# Reference: https://www.sammeechward.com/s3-and-iam-with-terraform

resource "aws_s3_bucket" "db_backup" {
  bucket = var.db_backup_bucket

  lifecycle {
    prevent_destroy = true
  }
}

resource "aws_s3_bucket_public_access_block" "db_backup_access" {
  bucket                  = aws_s3_bucket.db-backup.id
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

# IAM Policy
resource "aws_iam_policy" "db_backup_policy" {
  name        = var.db_backup_bucket_policy
  path        = "/"

  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Sid" : "VisualEditor0",
        "Effect" : "Allow",
        "Action" : [
          "s3:PutObject"
        ],
        "Resource" : [
          "arn:aws:s3:::${var.db_backup_bucket}/*"
        ]
      }
    ]
  })
}

# IAM Role
resource "aws_iam_role" "db_backup_role" {
  name = var.db_backup_bucket_role

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Sid    = ""
        Principal = {
          Service = "ec2.amazonaws.com"
        }
      },
    ]
  })
}

# Policy Attachment 
resource "aws_iam_role_policy_attachment" "db_backup_policy_attachment" {
  role       = aws_iam_role.db_backup_role.name
  policy_arn = aws_iam_policy.db_backup_policy.arn
}

# Instance Profile
resource "aws_iam_instance_profile" "db_backup_iam_instance_profile" {
  name = var.db_backup_bucket_iam_instance_profile
  role = aws_iam_role.some_role.name
}


