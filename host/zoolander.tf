output "zoolander" {
  value = aws_instance.zoolander.public_ip
}

resource "aws_instance" "zoolander" {
  ami             = var.ami.value
  instance_type   = "t2.micro"
  security_groups = [aws_security_group.zoolander.name]
  key_name        = aws_key_pair.zoolander.key_name

  tags = {
    Name = "zoolander"
  }

  connection {
    host = aws_instance.zoolander.public_ip
  }

  provisioner "remote-exec" {
    inline = ["git init zoolander"]
  }

  provisioner "file" {
    source      = "post-receive-hook.sh"
    destination = "zoolander/.git/hooks/post-receive"
  }

  provisioner "file" {
    source      = "nginx.conf"
    destination = "/etc/opt/ooce/nginx-1.16/nginx.conf"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x zoolander/.git/hooks/post-receive",
      "git config --system receive.denyCurrentBranch ignore",
      "svcadm enable svc:/network/http"
    ]
  }
}

variable "ami" {}

resource "aws_security_group" "zoolander" {
  name        = "zoolander"
  description = "SSH, HTTP, and outbound IP4 for Zoolander CI"

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = [join("/", [data.external.nat_ip.result.ip, 32])]
  }

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = [join("/", [data.external.nat_ip.result.ip, 32])]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "zoolander"
  }
}

data "external" "nat_ip" {
  program = ["bash", "-c", "echo '{'\\\"ip\\\": \\\"`dig -4 @resolver1.opendns.com myip.opendns.com +short`\\\"'}'"]
}

resource "aws_key_pair" "zoolander" {
  key_name   = "zoolander"
  public_key = file(pathexpand("~/.ssh/id_rsa.pub"))
}
