variable "instance_type" { type = string }
variable "api_image" { type = string }
variable "inquiry_table_arn" { type = string }
data "aws_ami" "al2023" { most_recent = true owners = ["amazon"] filter { name = "name" values = ["al2023-ami-2023*-arm64"] } filter { name = "virtualization-type" values = ["hvm"] } }
resource "aws_iam_role" "api" { name = "kloudlab-api-role" assume_role_policy = jsonencode({ Version = "2012-10-17" Statement = [{ Effect = "Allow" Principal = { Service = "ec2.amazonaws.com" } Action = "sts:AssumeRole" }] }) }
resource "aws_iam_role_policy" "api" { role = aws_iam_role.api.id policy = jsonencode({ Version = "2012-10-17" Statement = [{ Effect = "Allow" Action = ["dynamodb:PutItem", "dynamodb:GetItem"] Resource = var.inquiry_table_arn }, { Effect = "Allow" Action = ["ssm:GetParameter", "kms:Decrypt"] Resource = "*" }] }) }
resource "aws_iam_instance_profile" "api" { name = "kloudlab-api-profile" role = aws_iam_role.api.name }
resource "aws_instance" "api" { ami = data.aws_ami.al2023.id instance_type = var.instance_type iam_instance_profile = aws_iam_instance_profile.api.name user_data = <<-USERDATA
#!/bin/bash
echo "API image is managed by GitHub Actions: ${var.api_image}" > /etc/kloudlab-release
USERDATA lifecycle { ignore_changes = [user_data] } }
