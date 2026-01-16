# VirtualMac Project Guidelines

## Temporary Rules

### E2E Tests Moratorium
**DO NOT add new E2E tests to acceptance criteria until further notice.**

We are currently auditing the E2E test suite for redundant and low-value tests.
Until this cleanup is complete:
- Do not write new E2E tests
- Do not require E2E tests for PR acceptance
- Focus on unit tests if test coverage is needed
- Existing E2E test failures should still be fixed

This rule will be lifted once vi-t5a (E2E test cleanup) is complete.
