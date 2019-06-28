variable "ami" {}

resource "aws_instance" "zoolander" {
  ami           = var.ami.value
  instance_type = "t2.micro"
}
