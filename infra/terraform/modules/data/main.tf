variable "table_name" { type = string }
resource "aws_dynamodb_table" "inquiries" { name = var.table_name billing_mode = "PAY_PER_REQUEST" hash_key = "id" attribute { name = "id" type = "S" } point_in_time_recovery { enabled = true } server_side_encryption { enabled = true } }
output "table_name" { value = aws_dynamodb_table.inquiries.name }
output "table_arn" { value = aws_dynamodb_table.inquiries.arn }
