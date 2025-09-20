# Oracle Cloud Infrastructure Terraform configuration for JSON Oracle API

terraform {
  required_providers {
    oci = {
      source  = "oracle/oci"
      version = "~> 5.0"
    }
  }
}

# Configure the Oracle Cloud Infrastructure Provider
provider "oci" {
  region = var.region
}

# Variables
variable "region" {
  description = "Oracle Cloud region"
  type        = string
  default     = "us-ashburn-1"
}

variable "compartment_id" {
  description = "Compartment OCID"
  type        = string
}

variable "availability_domain" {
  description = "Availability Domain"
  type        = string
  default     = "AD-1"
}

variable "ssh_public_key_path" {
  description = "Path to SSH public key"
  type        = string
  default     = "~/.ssh/id_rsa.pub"
}

# Data sources
data "oci_identity_availability_domains" "ads" {
  compartment_id = var.compartment_id
}

data "oci_core_images" "ubuntu_images" {
  compartment_id   = var.compartment_id
  operating_system = "Canonical Ubuntu"
  sort_by          = "TIMECREATED"
  sort_order       = "DESC"
}

data "oci_core_vcns" "vcn" {
  compartment_id = var.compartment_id
}

data "oci_core_subnets" "subnets" {
  compartment_id = var.compartment_id
  vcn_id         = data.oci_core_vcns.vcn.virtual_networks[0].id
}

# Security Group (Security List)
resource "oci_core_security_list" "json_oracle_security_list" {
  compartment_id = var.compartment_id
  vcn_id         = data.oci_core_vcns.vcn.virtual_networks[0].id
  display_name   = "json-oracle-security-list"

  ingress_security_rules {
    protocol  = "6" # TCP
    source    = "0.0.0.0/0"
    stateless = false

    tcp_options {
      min = 22
      max = 22
    }
  }

  ingress_security_rules {
    protocol  = "6" # TCP
    source    = "0.0.0.0/0"
    stateless = false

    tcp_options {
      min = 3000
      max = 3000
    }
  }

  ingress_security_rules {
    protocol  = "6" # TCP
    source    = "0.0.0.0/0"
    stateless = false

    tcp_options {
      min = 80
      max = 80
    }
  }

  ingress_security_rules {
    protocol  = "6" # TCP
    source    = "0.0.0.0/0"
    stateless = false

    tcp_options {
      min = 443
      max = 443
    }
  }

  egress_security_rules {
    protocol    = "all"
    destination = "0.0.0.0/0"
    stateless   = false
  }
}

# Compute Instance
resource "oci_core_instance" "json_oracle_instance" {
  compartment_id      = var.compartment_id
  availability_domain = data.oci_identity_availability_domains.ads.availability_domains[0].name
  display_name        = "json-oracle-api"
  shape               = "VM.Standard.E2.1.Micro" # Always Free eligible

  source_details {
    source_type = "image"
    source_id   = data.oci_core_images.ubuntu_images.images[0].id
  }

  create_vnic_details {
    subnet_id        = data.oci_core_subnets.subnets.subnets[0].id
    display_name     = "json-oracle-vnic"
    assign_public_ip = true
    hostname_label   = "json-oracle"
  }

  metadata = {
    ssh_authorized_keys = file(var.ssh_public_key_path)
    user_data = base64encode(templatefile("${path.module}/user-data.sh", {
      api_port = 3000
    }))
  }

  agent_config {
    plugins_config {
      desired_state = "ENABLED"
      name          = "Vulnerability Scanning"
    }
    plugins_config {
      desired_state = "ENABLED"
      name          = "OS Management Service Agent"
    }
    plugins_config {
      desired_state = "ENABLED"
      name          = "Compute Instance Monitoring"
    }
  }
}

# Load Balancer
resource "oci_load_balancer_load_balancer" "json_oracle_lb" {
  compartment_id = var.compartment_id
  display_name   = "json-oracle-lb"
  shape          = "flexible"
  shape_details {
    minimum_bandwidth_in_mbps = 10
    maximum_bandwidth_in_mbps = 100
  }

  subnet_ids = [
    data.oci_core_subnets.subnets.subnets[0].id
  ]
}

# Backend Set
resource "oci_load_balancer_backend_set" "json_oracle_backend_set" {
  load_balancer_id = oci_load_balancer_load_balancer.json_oracle_lb.id
  name             = "json-oracle-backend-set"
  policy           = "ROUND_ROBIN"

  health_checker {
    protocol          = "HTTP"
    port              = 3000
    url_path          = "/health"
    interval_ms       = 10000
    timeout_in_millis = 3000
    retries           = 3
  }
}

# Backend
resource "oci_load_balancer_backend" "json_oracle_backend" {
  load_balancer_id = oci_load_balancer_load_balancer.json_oracle_lb.id
  backendset_name  = oci_load_balancer_backend_set.json_oracle_backend_set.name
  ip_address       = oci_core_instance.json_oracle_instance.public_ip
  port             = 3000
  backup           = false
  drain            = false
  offline          = false
  weight           = 1
}

# Listener
resource "oci_load_balancer_listener" "json_oracle_listener" {
  load_balancer_id         = oci_load_balancer_load_balancer.json_oracle_lb.id
  name                     = "json-oracle-listener"
  default_backend_set_name = oci_load_balancer_backend_set.json_oracle_backend_set.name
  port                     = 80
  protocol                 = "HTTP"

  connection_configuration {
    idle_timeout_in_seconds = "2400"
  }
}

# Outputs
output "instance_public_ip" {
  description = "Public IP address of the instance"
  value       = oci_core_instance.json_oracle_instance.public_ip
}

output "load_balancer_ip" {
  description = "Load balancer IP address"
  value       = oci_load_balancer_load_balancer.json_oracle_lb.ip_addresses[0].ip_address
}

output "api_url" {
  description = "API URL"
  value       = "http://${oci_load_balancer_load_balancer.json_oracle_lb.ip_addresses[0].ip_address}"
}
