# Varina Backend Utilisation

- [Retour à Index de Documentation](TOC.md)

## Purpose

Varina backend handles product-specific commands and workflows. It communicates exclusively with the Engine and emits events for UI consumption.

## Operation

- Typically spawned and supervised by the Engine.
- Exposes commands through the protocol, not direct HTTP.

## Notes

For architecture context, see `documentation/technical_documentation/ARCHITECTURE.md`.
