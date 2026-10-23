# ADR-0003: Separate commercial catalog from resource inventory

**Status:** Accepted (subject to review)  
**Date:** 2026-10-23  

## Context
Confusion between "what we sell" vs "what we have".

## Decision
Catalog owns products/offers/rate plans; Inventory owns devices/SIM/eSIM instances/assignments.

## Consequences
+ Clear boundaries, correct modeling
- Need integration for provisioning
