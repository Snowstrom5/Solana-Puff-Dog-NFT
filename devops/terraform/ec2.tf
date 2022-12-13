resource "aws_instance" "climate-news-service" {
  ami                    = var.aws_base_ami
  instance_type          = var.aws_instance_type
  key_name               = var.aws_key_name
  availability_zone      = var.aws_availability_zone
  vpc_security_group_ids = [aws_security_group.main.id]

  lifecycle {
    prevent_destroy = true
  }

  root_block_device {
    volume_size = 100
    volume_type = "gp3"
  }

  connection {
    type        = "ssh"
    user        = "ubuntu"
    private_key = file("${var.SSH_FOLDER}/${var.aws_pem_file}")
    host        = self.public_ip
  }

  # Insalls Ansible on the server
  provisioner "remote-exec" {
    inline = [
      "sudo apt-add-repository ppa:ansible/ansible -y",
      "sudo apt update",
      "sudo apt install ansible -y",
    ]
  }

}

resource "aws_eip" "elastic_ip" {
  instance = aws_instance.climate-news-service.id
  vpc      = true
}

# Security Group 
resource "aws_security_group" "main" {
  egress = [
    {
      cidr_blocks      = ["0.0.0.0/0"]
      description      = ""
      from_port        = 0
      ipv6_cidr_blocks = []
      prefix_list_ids  = []
      protocol         = "-1"
      security_groups  = []
      self             = false
      to_port          = 0
    }
  ]
  ingress = [
    # An IP that is allowed to ssh into the host.
    {
      cidr_blocks      = ["${var.MY_IP_ADDRESS}/32"]
      description      = ""
      from_port        = 22
      ipv6_cidr_blocks = []
      prefix_list_ids  = []
      protocol         = "tcp"
      security_groups  = []
      self             = false
      to_port          = 22
    },
    # SSL
    {
      cidr_blocks      = ["0.0.0.0/0", ]
      description      = ""
      from_port        = 443
      ipv6_cidr_blocks = []
      prefix_list_ids  = []
      protocol         = "tcp"
      security_groups  = []
      self             = false
      to_port          = 443
    },
    # HTTP
    {
      cidr_blocks      = ["0.0.0.0/0", ]
      description      = ""
      from_port        = 80
      ipv6_cidr_blocks = []
      prefix_list_ids  = []
      protocol         = "tcp"
      security_groups  = []
      self             = false
      to_port          = 80
    }
  ]
}

