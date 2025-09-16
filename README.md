# Community Manager

A decentralized community management platform enabling local communities to organize, govern, and collaborate with end-to-end encrypted communication.

## Features
- Multi-role community management (owner, socio, investor, affiliate, supporter)
- Geographic-based community discovery and creation
- End-to-end encrypted real-time chat
- Local business integration with geographic mapping
- Decentralized governance tools (polls, voting, decision-making)
- Mobile-first design (web + future React Native app)

## Architecture
- **Frontend**: Next.js 14 with TypeScript and Material UI (Vercel deployment)
- **Backend**: Rust microservices with cargo-lambda (Lambda → EC2 progression)
- **Database**: CockroachDB Serverless (PostgreSQL-compatible)
- **Authentication**: Auth0 with custom role management
- **Chat**: Stateless WebSocket service with ephemeral message storage
- **Infrastructure**: AWS with progressive scaling (Lambda → EC2 Spot)

## Development Phases
1. **Foundation** (4-6 weeks): Auth, communities, basic roles
2. **Core Features** (3-4 weeks): Chat, business profiles, maps
3. **Advanced** (3-4 weeks): E2EE, governance, advanced roles
4. **Mobile** (4-5 weeks): React Native app development

## Quick Start
```bash
./scripts/setup.sh      # Initial setup
./scripts/dev.sh        # Start development environment
```

## Architecture Evolution
- **Phase 1**: Lambda + API Gateway (~$15/month)
- **Phase 2**: Lambda + EC2 WebSocket (~$40/month)
- **Phase 3**: Direct ALB WebSocket (~$25/month)

## Development
- **Backend**: Rust with cargo-lambda for agile deployment
- **Frontend**: Next.js 14 with Material UI for rapid development
- **Database**: CockroachDB with PostGIS-compatible geographic features
- **Real-time**: WebSocket with ephemeral message queuing

## Deployment
```bash
# Deploy to development
./scripts/deploy.sh dev all

# Deploy individual services
./scripts/deploy.sh dev api
./scripts/deploy.sh dev chat
```

## Project Structure
```
community-manager/
├── backend/           # Rust microservices
│   ├── api-gateway/   # REST API service
│   ├── chat-service/  # WebSocket chat service
│   ├── shared/        # Common Rust code
│   └── migrations/    # Database migrations
├── frontend/          # Next.js + Material UI app
├── scripts/           # Development and deployment automation
└── docs/              # Documentation
```

## Contributing
1. Install prerequisites: Rust, Node.js, Docker
2. Run `./scripts/setup.sh` for initial setup
3. Start development with `./scripts/dev.sh`
4. Follow conventional commit messages
5. Test before submitting PRs

## License
MIT License - see LICENSE file for details