# Agent Instructions & Mandatory Skills Enforcement

This repository (`tam-traffic-monitoring`) has 16 specialized skills installed in `.agents/skills/`. The agent **MUST ALWAYS** proactively adhere to and apply these skills across all interactions, planning, architecture design, coding, testing, review, and deployment activities.

## Installed Skills & Application Workflow

### 1. Workflow Initiation & Planning
- **`using-superpowers` (`.agents/skills/using-superpowers`)**:
  - Always check relevant skills at the start of any conversation and task.
  - Prioritize skill workflows over generic improvisation.
- **`writing-plans` (`.agents/skills/writing-plans`)**:
  - For any multi-step task, feature implementation, or refactoring, produce a clear, phased implementation plan with verifiable criteria before modifying code.
- **`grill-me` (`.agents/skills/grill-me`)**:
  - Proactively identify ambiguities, trade-offs, and critical decisions; engage in structured questioning to sharpen architecture and requirements.
- **`find-skills` (`.agents/skills/find-skills`)**:
  - Check for existing or installable skills whenever encountering novel requirements.

### 2. Architecture & Codebase Design
- **`codebase-design` (`.agents/skills/codebase-design`)**:
  - Design deep modules with narrow, expressive interfaces and powerful internal implementations.
  - Place seams thoughtfully to maximize testability and maintainability.
- **`improve-codebase-architecture` (`.agents/skills/improve-codebase-architecture`)**:
  - Proactively scan the codebase for deepening opportunities, module boundaries, and architectural simplification.

### 3. Implementation & Test-Driven Development (TDD)
- **`tdd` (`.agents/skills/tdd`)**:
  - Always write tests first (Red-Green-Refactor) when building new features, business logic, or fixing bugs.
  - Never ship untested code. Ensure high unit and integration test coverage.
- **`no-ai-slop` (`.agents/skills/no-ai-slop`)**:
  - Strictly avoid generic AI fluff, obvious/redundant code comments, boilerplate prose, and robotic language. Keep code and explanations concise, direct, and purposeful.
- **`caveman` (`.agents/skills/caveman`)**:
  - Be terse, eliminate filler words, lead with technical facts, and avoid conversational preamble.

### 4. UI / UX Design & Frontend Engineering
- **`ui-ux-pro-max` (`.agents/skills/ui-ux-pro-max`)**:
  - For any frontend, UI, or dashboard work, apply high-end UI/UX standards: cohesive palettes, intentional typography, responsive layouts, micro-interactions, accessibility (WCAG), and clean component architecture.

### 5. Quality Assurance & E2E Testing
- **`e2e-testing-patterns` (`.agents/skills/e2e-testing-patterns`)**:
  - Implement robust end-to-end tests using Playwright/Cypress.
  - Follow best practices: stable locators (data-testid, role/text), isolated test states, and avoidance of flaky assertions.

### 6. Security & Vulnerability Audits
- **`security-review` (`.agents/skills/security-review`)**:
  - Systematically review code for OWASP Top 10 vulnerabilities, injection flaws, authentication/authorization issues, and cryptographic weaknesses.
- **`vibe-security` (`.agents/skills/vibe-security`)**:
  - Guard against common AI-generated security traps: exposed secrets, permissive database/API access controls, insecure client-side logic, and unvalidated inputs.

### 7. Systems & Operations
- **`sysadmin` (`.agents/skills/sysadmin`)**:
  - Apply battle-tested DevOps and system administration practices for daemon services, background processes, containerization, and monitoring.

### 8. Documentation & Standards
- **`writing-guidelines` (`.agents/skills/writing-guidelines`)**:
  - Maintain consistent, high-standard technical documentation, READMEs, and API specs.
- **`writing-skills` (`.agents/skills/writing-skills`)**:
  - When creating or modifying agent skills, adhere to standardized SKILL.md schemas, triggering conditions, and progressive disclosure principles.

