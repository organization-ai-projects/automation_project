# Overview

## Introduction

This document provides an overview of the `automation_project`, an advanced automation workspace. For specific details, see the following sections:

- [Multi-project Organization](projects/organization.md)
- [Non-Negotiable Principles](principles.md)
- [Automated Documentation](documentation.md)
- [Workflow Orchestration](workflows.md)
- [Consolidated Planning](planning.md)

This overview does not describe implementation details, which are deliberately covered in specialized documents.

---

## 1. Objective

The goal of this project is to build an **advanced automation workspace** (similar to Google/Microsoft) capable of orchestrating **multiple simultaneous projects**, with advanced automation of the software development lifecycle.

The system aims to automate:

- Code generation
- Linting and structural validation
- Documentation
- Application and evolution of best practices
- Orchestration of complete workflows (analysis → generation → validation → iteration)

The project is **100% Rust**, with:

- a strong **symbolic foundation** (rules, structures, invariants)
- an **optional and activatable neural component** (Burn)

---

## 2. Fundamental Concept: `automation_project`

### 2.1 Definition

**`automation_project` is the root workspace.**

It is not a simple project, but a **global tooled environment**, capable of managing **N heterogeneous projects**.

> `automation_project` = root container + registry + tooling + orchestration

It can contain:

- **Final products** (applications, services, tools)
- **Libraries / SDKs** (reusable, versioned)
