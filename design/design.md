# Qurve design document

This document outline the high-level design of Qurve and can be used as a starting document by contributors.

## Terminology

You must be aware of our terminology so you know where to look at:

- [**Atom**](#atom): An atom is a single runnable resource (e.g container, serverless function).
- [**Trait**](#traits): A declarative characteristic of a given atom. See some examples:
  - Expose trait: Will expose the atom publicly.
  - Autoscale trait: Add autoscaling properties to your atom.
  - Monitoring trait: Attach a monitoring to your atom.
- [**Resolver**](#resolver): A small module responsible for resolving a `trait` for a given `atom` in a `Qurvefile`
- [**Converge**](#converge): Use the resolvers in the `Qurvefile` to converge the state (e.g deploy container in kubernetes, deploy serverless function)
- [**Qurvefile**](#qurvefile): A declarative file containing all the specs for a given service/resource.
- [**Qurve**](#qurve): A qurve is a self-contained artifact that can be consumed by the `Qurve engine` to converge.
- [**Qurve Engine**](#engine): Ingest a `qurve` and converge.

### Atom

An atom is a deployable single-unit, for example it could be a container in the context of Kubernetes, it could be
a Serverless function or more...
An atom is described in a declarative and agnostic way, instead of saying:

- I want a Kubernetes pod
- I want a Kubernetes deployment
- I want an AWS Lambda

You say:

- I want a serverless runtime
- I want my atom to be monitored
- I want my atom to auto scale