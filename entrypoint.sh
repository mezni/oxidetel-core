#!/bin/sh
/app/target/debug/auth_service &
/app/target/debug/crm_service &
/app/target/debug/billing_service &
/app/target/debug/inventory_service &
/app/target/debug/ticketing_service &
/app/target/debug/catalog_service &
wait
