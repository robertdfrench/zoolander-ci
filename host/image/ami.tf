# Fetch latest Zoolander AMI
data "aws_ami" "ami" {
  most_recent = true
  owners      = ["self"]
  name_regex  = "^zoolander-*"
}

# Report its id
output "ami" {
  value = data.aws_ami.ami.id
}
