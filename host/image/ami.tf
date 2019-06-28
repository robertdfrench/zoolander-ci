data "aws_ami" "ami" {
  most_recent = true
  owners      = ["self"]
  name_regex  = "^zoolander-omnios *"
}

output "ami" {
  value = data.aws_ami.ami.id
}
