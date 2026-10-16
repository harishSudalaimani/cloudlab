variable "domain_name" { type = string }
resource "aws_s3_bucket" "frontend" { bucket = "${var.domain_name}-frontend" }
resource "aws_s3_bucket_public_access_block" "frontend" { bucket = aws_s3_bucket.frontend.id block_public_acls = true block_public_policy = true ignore_public_acls = true restrict_public_buckets = true }
resource "aws_cloudfront_origin_access_control" "frontend" { name = "${var.domain_name}-oac" description = "CloudFront access to private frontend bucket" origin_access_control_origin_type = "s3" signing_behavior = "always" signing_protocol = "sigv4" }
output "bucket_name" { value = aws_s3_bucket.frontend.bucket }
