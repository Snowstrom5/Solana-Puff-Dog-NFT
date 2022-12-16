# Route 53 zone
resource "aws_route53_zone" "production" {
  name = var.base_domain_name
}

# www CNAME record
resource "aws_route53_record" "cname_www" {
  zone_id = aws_route53_zone.production.id
  name    = "www.${var.base_domain_name}"
  type    = "CNAME"
  ttl     = 3600
  records = [var.base_domain_name]
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

# TXT record - Email & Google site verification
resource "aws_route53_record" "txt" {
  zone_id = aws_route53_zone.production.id
  name    = var.base_domain_name
  type    = "TXT"
  ttl     = 300
  records = ["v=spf1 include:spf.privateemail.com ~all", "google-site-verification=qkOoX9PBrekUIPcRDFsG0fFvALzzWwVNcg4QXl2XuBI"]
}

# A record pointing to elastic ip
resource "aws_route53_record" "www" {
  zone_id = aws_route53_zone.production.id
  name    = var.base_domain_name
  type    = "A"
  ttl     = 300
  records = [aws_eip.elastic_ip.public_ip]
}
