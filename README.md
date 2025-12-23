# jammin + JAM SDK Example

This project was created with [jammin](https://github.com/FluffyLabs/jammin), FluffyLabs' toolbox for Typeberry builders.

## What is jammin?

jammin is a comprehensive toolkit that helps you spin up projects, build services, ship them to a network, and monitor what happens after deploy. This template uses the **JAM SDK** for building JAM services.

## Getting Started

This project includes:
- Pre-configured JAM SDK service in `services/example`
- Build configuration via `jammin.build.yml`
- Ready-to-use development environment

## Available Commands

### Build Services

```bash
jammin build
```

Builds all services defined in your `jammin.build.yml` configuration.

### Run Tests

```bash
jammin test
```

Runs unit tests for your services.

### Deploy

```bash
jammin deploy
```

Deploys your services to a network.

## Project Structure

```
.
├── jammin.build.yml    # jammin configuration
└── services/
    └── example/        # Your JAM SDK service
```

## Learn More

- [jammin on github](https://github.com/FluffyLabs/jammin)
- [jammin on npm](https://www.npmjs.com/package/@fluffylabs/jammin)

## Next Steps

1. Explore the example service in `services/example/`
2. Run `jammin build` to build your service
3. Customize the service to fit your needs
4. Deploy to a testnet with `jammin deploy`
