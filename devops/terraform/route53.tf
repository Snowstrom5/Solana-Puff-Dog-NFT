# Route 53 zone
resource "aws_route53_zone" "production" {
  name = var.base_domain_name
}

# MX record
resource "aws_route53_record" "mx" {
  zone_id = aws_route53_zone.production.id
  name    = var.base_domain_name
  type    = "MX"
  ttl     = 3600
  records = [
    "10 mx1.privateemail.com",
    "10 mx2.privateemail.com"
  ]
}

# TXT record
resource "aws_route53_record" "txt" {
  zone_id = aws_route53_zone.production.id
  name    = var.base_domain_name
  type    = "TXT"
  ttl     = 300
  records = ["v=spf1 include:spf.privateemail.com ~all"]
}

# A record pointing to elastic ip
resource "aws_route53_record" "www" {
  zone_id = aws_route53_zone.production.id
  name    = var.base_domain_name
  type    = "A"
  ttl     = 300
  records = [aws_eip.elastic_ip.public_ip]
}
