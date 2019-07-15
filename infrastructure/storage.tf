locals {
  storage = {
    left  = aws_ebs_volume.left
    right = aws_ebs_volume.right
  }
}

resource "aws_ebs_volume" "left" {
  availability_zone = local.az
  size              = local.mirror_size

  tags = {
    Name = "zoolander-left"
  }
}

resource "aws_ebs_volume" "right" {
  availability_zone = local.az
  size              = local.mirror_size

  tags = {
    Name = "zoolander-right"
  }
}

locals {
  az          = format("%sa", data.aws_region.current.name)
  mirror_size = 1
}

data "aws_region" "current" {}
