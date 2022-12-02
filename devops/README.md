### Prerequisites
To enable ssh access from an IP address, set the following env variable to add an IP address to the security group.
```bash
export TF_VAR_MY_IP_ADDRESS="x.x.x.x"
```

### Terraform 
```bash
cd terraform
sudo terraform apply
```

### Ansible

```bash
cd ansible
ansible-playbook playbooks/docker.yml 
```

