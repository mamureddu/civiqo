# Chat Service

Real-time WebSocket service for end-to-end encrypted messaging with progressive scaling architecture.

## Overview

The Chat Service provides:
- E2EE message routing (no permanent storage)
- Connection management across instances
- Presence indicators and typing status
- Ephemeral offline message queuing (24h TTL)
- Room management and permissions
- Key exchange coordination for E2EE

## Architecture

### Phase 1: Lambda WebSocket
- **Deployment**: AWS Lambda with API Gateway WebSocket
- **Scaling**: Serverless auto-scaling
- **State**: Stateless with database connection tracking

### Phase 2: EC2 + API Gateway WebSocket
- **Deployment**: EC2 Spot instances behind ALB
- **WebSocket**: Still via API Gateway WebSocket API
- **Scaling**: Auto Scaling Groups with spot instances

### Phase 3: Direct ALB WebSocket
- **Deployment**: EC2 Spot instances behind ALB
- **WebSocket**: Direct WebSocket handling (bypass API Gateway)
- **Cost**: ~90% savings vs API Gateway WebSocket

## E2EE Design

### Key Principles
- **No message storage**: Messages only stored temporarily in SQS for offline users
- **Client-side encryption**: All encryption/decryption happens in browser/mobile
- **Public key storage**: Only public keys stored in database
- **Ephemeral queuing**: Messages auto-expire after 24 hours

### Message Flow
```
User A → Encrypt → WebSocket → Route via SNS → WebSocket → User B → Decrypt
                        ↓
              (if User B offline)
                        ↓
                  SQS Queue (24h TTL)
```

## Cargo Lambda Configuration

### Development (Lambda)
```bash
cargo lambda deploy --config-file deploy-dev.toml
```

### Production (Lambda)
```bash
cargo lambda deploy --config-file deploy-prod.toml
```

### EC2 Scaling (Future)
```bash
# Will use deploy-ec2.toml for EC2 deployment
# Terraform/CloudFormation for infrastructure
```

## Environment Configuration

### Development (dev.env)
- Local PostgreSQL for connection tracking
- LocalStack SQS/SNS endpoints
- Debug logging and development flags
- Longer timeouts for debugging

### Production (prod.env)
- CockroachDB production cluster
- Real AWS SQS/SNS services
- Optimized timeouts and batch sizes
- Production logging levels

## WebSocket Message Types

### Connection Management
- `Connect` - User establishes WebSocket connection
- `Disconnect` - User disconnects
- `Heartbeat` - Keep connection alive

### Room Management
- `JoinRoom` - Join chat room
- `LeaveRoom` - Leave chat room

### Messaging
- `SendMessage` - Send encrypted message
- `ReceiveMessage` - Receive encrypted message

### Presence & Typing
- `UserPresence` - Online/offline status
- `TypingStart` - User starts typing
- `TypingStop` - User stops typing

### Key Exchange
- `KeyExchange` - Exchange public keys for E2EE

## Stateless Design

### Connection Tracking
```rust
// In-memory per instance
connections: HashMap<UserId, ConnectionInfo>

// Cross-instance via database
active_connections table with TTL cleanup
```

### Message Routing
```rust
1. Message arrives → Check local connections
2. If not local → Publish to SNS topic
3. All instances receive → Check their connections
4. Owner instance delivers → Remove from queue
5. No active connection → Queue in SQS with TTL
```

## Scaling Strategy

### Lambda Phase (Current)
- Cold start optimization
- Connection state in database
- SNS fan-out for routing

### EC2 Phase (Growth)
- Warm instances reduce latency
- Auto Scaling based on connections
- Spot instances for cost optimization

### Direct WebSocket Phase (Scale)
- Custom WebSocket handling
- Maximum performance and control
- Significant cost savings

## Security

### Authentication
- JWT validation on connect
- User ID extraction from token
- Room membership verification

### Authorization
- Community membership checks
- Room-level permissions
- Rate limiting per user

### E2EE Implementation
- Client generates key pairs
- Public keys stored in database
- Private keys never leave client
- Message content encrypted before WebSocket

## Performance

### Connection Limits
- Development: 5 connections per user
- Production: 3 connections per user
- Auto-cleanup of stale connections

### Message Handling
- Batch processing for offline messages
- Configurable message size limits
- TTL-based cleanup processes

### Resource Management
- Connection pooling
- Memory usage optimization
- Graceful connection shutdown

## Monitoring

### Metrics
- Active connections count
- Message throughput
- Connection duration
- Error rates

### Logging
- Connection events
- Message routing
- Error conditions
- Performance metrics

## Local Development

```bash
# Start with hot reloading
cargo lambda watch --env-file lambda-env/dev.env --port 9002

# Test WebSocket connection
wscat -c ws://localhost:9002
```

## Deployment

```bash
# Build
cargo lambda build --release

# Deploy to development
cargo lambda deploy --config-file deploy-dev.toml

# Deploy via script
../../../scripts/deploy.sh dev chat
```

## Migration Path

### Phase 1 → Phase 2
1. Deploy EC2 infrastructure
2. Route traffic to both Lambda and EC2
3. Gradually increase EC2 traffic
4. Monitor performance and costs

### Phase 2 → Phase 3
1. Implement direct WebSocket handling
2. Deploy custom WebSocket service
3. Update ALB target groups
4. Remove API Gateway WebSocket dependency

Each phase maintains backward compatibility and zero-downtime deployment.