resource "aws_eip" "ip" {
  vpc = false
}

output "ip" {
  value = {
    id  = aws_eip.ip.id
    ip  = aws_eip.ip.public_ip
    dns = aws_eip.ip.public_dns
  }
}
