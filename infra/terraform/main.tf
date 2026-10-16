module "frontend" { source = "./modules/frontend" domain_name = var.domain_name }
module "data" { source = "./modules/data" table_name = "kloudlab-inquiries" }
module "api" { source = "./modules/api" instance_type = var.api_instance_type api_image = var.api_image inquiry_table_arn = module.data.table_arn }
output "frontend_bucket" { value = module.frontend.bucket_name }
output "inquiry_table" { value = module.data.table_name }
