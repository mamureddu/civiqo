'use client';

import { useState, useEffect, useRef } from 'react';
import { useUser } from '@auth0/nextjs-auth0';
import {
  Box,
  Container,
  Paper,
  Grid,
  List,
  ListItem,
  ListItemText,
  ListItemAvatar,
  TextField,
  IconButton,
  Typography,
  Avatar,
  Chip,
  Divider,
  Badge,
  Stack,
  Alert,
  CircularProgress,
} from '@mui/material';
import {
  Send as SendIcon,
  Chat as ChatIcon,
  Groups as GroupsIcon,
  Person as PersonIcon,
  VpnLock as EncryptionIcon,
} from '@mui/icons-material';
import DashboardLayout from '@/components/layout/DashboardLayout';
import { formatDistanceToNow } from 'date-fns';

interface ChatRoom {
  id: string;
  name: string;
  type: 'community' | 'direct';
  participants: number;
  lastMessage?: string;
  lastMessageTime?: Date;
  unreadCount: number;
  avatar?: string;
}

interface Message {
  id: string;
  content: string;
  senderId: string;
  senderName: string;
  senderAvatar?: string;
  timestamp: Date;
  encrypted: boolean;
}

// Mock data for demonstration
const mockRooms: ChatRoom[] = [
  {
    id: '1',
    name: 'Downtown Community',
    type: 'community',
    participants: 245,
    lastMessage: 'Thanks for organizing the cleanup event!',
    lastMessageTime: new Date(Date.now() - 1000 * 60 * 30), // 30 minutes ago
    unreadCount: 3,
  },
  {
    id: '2',
    name: 'Local Business Owners',
    type: 'community',
    participants: 52,
    lastMessage: 'The new restaurant on Main St is excellent',
    lastMessageTime: new Date(Date.now() - 1000 * 60 * 60 * 2), // 2 hours ago
    unreadCount: 0,
  },
  {
    id: '3',
    name: 'Sarah Johnson',
    type: 'direct',
    participants: 2,
    lastMessage: 'See you at the meeting tomorrow',
    lastMessageTime: new Date(Date.now() - 1000 * 60 * 60 * 24), // 1 day ago
    unreadCount: 1,
  },
];

const mockMessages: Message[] = [
  {
    id: '1',
    content: 'Hey everyone! Thanks for joining the Downtown Community chat.',
    senderId: 'user1',
    senderName: 'Alex Johnson',
    timestamp: new Date(Date.now() - 1000 * 60 * 60 * 2),
    encrypted: true,
  },
  {
    id: '2',
    content: 'Great to be here! Looking forward to getting involved in local activities.',
    senderId: 'user2',
    senderName: 'Maria Garcia',
    timestamp: new Date(Date.now() - 1000 * 60 * 60),
    encrypted: true,
  },
  {
    id: '3',
    content: 'Does anyone know when the next community meeting is scheduled?',
    senderId: 'user3',
    senderName: 'David Chen',
    timestamp: new Date(Date.now() - 1000 * 60 * 45),
    encrypted: true,
  },
  {
    id: '4',
    content: 'The next meeting is Thursday at 7 PM at the community center.',
    senderId: 'user1',
    senderName: 'Alex Johnson',
    timestamp: new Date(Date.now() - 1000 * 60 * 30),
    encrypted: true,
  },
];

export default function ChatPage() {
  const { user } = useUser();
  const [selectedRoom, setSelectedRoom] = useState<ChatRoom | null>(mockRooms[0]);
  const [messages, setMessages] = useState<Message[]>(mockMessages);
  const [newMessage, setNewMessage] = useState('');
  const [isConnecting, setIsConnecting] = useState(false);
  const [connectionError, setConnectionError] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  // Simulate WebSocket connection
  useEffect(() => {
    if (selectedRoom) {
      setIsConnecting(true);
      setConnectionError(null);

      // Simulate connection delay
      const timer = setTimeout(() => {
        setIsConnecting(false);
      }, 1000);

      return () => clearTimeout(timer);
    }
  }, [selectedRoom]);

  const handleSendMessage = () => {
    if (newMessage.trim() && selectedRoom && user) {
      const message: Message = {
        id: Date.now().toString(),
        content: newMessage.trim(),
        senderId: user.sub || 'current-user',
        senderName: user.name || user.email || 'You',
        senderAvatar: user.picture,
        timestamp: new Date(),
        encrypted: true,
      };

      setMessages(prev => [...prev, message]);
      setNewMessage('');
    }
  };

  const handleKeyPress = (event: React.KeyboardEvent) => {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSendMessage();
    }
  };

  const RoomListItem = ({ room }: { room: ChatRoom }) => (
    <ListItem
      button
      selected={selectedRoom?.id === room.id}
      onClick={() => setSelectedRoom(room)}
      sx={{
        borderRadius: 1,
        mb: 1,
        '&:hover': {
          bgcolor: 'action.hover',
        },
        '&.Mui-selected': {
          bgcolor: 'primary.light',
          '&:hover': {
            bgcolor: 'primary.light',
          },
        },
      }}
    >
      <ListItemAvatar>
        <Badge badgeContent={room.unreadCount} color="error">
          <Avatar>
            {room.type === 'community' ? <GroupsIcon /> : <PersonIcon />}
          </Avatar>
        </Badge>
      </ListItemAvatar>
      <ListItemText
        primary={room.name}
        secondary={
          <Box>
            <Typography variant="body2" color="text.secondary" noWrap>
              {room.lastMessage}
            </Typography>
            <Stack direction="row" spacing={1} alignItems="center" mt={0.5}>
              <Typography variant="caption" color="text.secondary">
                {room.participants} participants
              </Typography>
              {room.lastMessageTime && (
                <Typography variant="caption" color="text.secondary">
                  • {formatDistanceToNow(room.lastMessageTime)} ago
                </Typography>
              )}
            </Stack>
          </Box>
        }
      />
    </ListItem>
  );

  const MessageBubble = ({ message, isOwn }: { message: Message; isOwn: boolean }) => (
    <Box
      sx={{
        display: 'flex',
        justifyContent: isOwn ? 'flex-end' : 'flex-start',
        mb: 2,
      }}
    >
      <Stack
        direction={isOwn ? 'row-reverse' : 'row'}
        spacing={1}
        alignItems="flex-end"
        sx={{ maxWidth: '70%' }}
      >
        {!isOwn && (
          <Avatar
            src={message.senderAvatar}
            sx={{ width: 32, height: 32 }}
          >
            {message.senderName.charAt(0)}
          </Avatar>
        )}

        <Box>
          {!isOwn && (
            <Typography variant="caption" color="text.secondary" sx={{ ml: 1 }}>
              {message.senderName}
            </Typography>
          )}

          <Paper
            sx={{
              p: 2,
              bgcolor: isOwn ? 'primary.main' : 'grey.100',
              color: isOwn ? 'primary.contrastText' : 'text.primary',
              borderRadius: 2,
              borderTopLeftRadius: isOwn ? 2 : 0.5,
              borderTopRightRadius: isOwn ? 0.5 : 2,
            }}
          >
            <Typography variant="body2">
              {message.content}
            </Typography>
          </Paper>

          <Stack direction="row" spacing={1} alignItems="center" sx={{ mt: 0.5, ml: 1 }}>
            <Typography variant="caption" color="text.secondary">
              {formatDistanceToNow(message.timestamp)} ago
            </Typography>
            {message.encrypted && (
              <EncryptionIcon sx={{ fontSize: 12, color: 'success.main' }} />
            )}
          </Stack>
        </Box>
      </Stack>
    </Box>
  );

  return (
    <DashboardLayout>
      <Container maxWidth="xl" sx={{ height: 'calc(100vh - 200px)' }}>
        {/* Header */}
        <Box mb={3}>
          <Typography variant="h4" component="h1" fontWeight="bold" gutterBottom>
            Secure Chat
          </Typography>
          <Stack direction="row" spacing={1} alignItems="center">
            <EncryptionIcon sx={{ fontSize: 16, color: 'success.main' }} />
            <Typography variant="body2" color="text.secondary">
              End-to-end encrypted • Messages are not stored on servers
            </Typography>
          </Stack>
        </Box>

        <Grid container sx={{ height: '100%' }}>
          {/* Chat Rooms Sidebar */}
          <Grid item xs={12} md={4} lg={3}>
            <Paper sx={{ height: '100%', p: 2 }}>
              <Typography variant="h6" fontWeight="bold" gutterBottom>
                Chat Rooms
              </Typography>

              <List sx={{ maxHeight: 'calc(100% - 60px)', overflow: 'auto' }}>
                {mockRooms.map((room) => (
                  <RoomListItem key={room.id} room={room} />
                ))}
              </List>
            </Paper>
          </Grid>

          {/* Chat Area */}
          <Grid item xs={12} md={8} lg={9}>
            <Paper sx={{ height: '100%', display: 'flex', flexDirection: 'column', ml: { md: 2 } }}>
              {selectedRoom ? (
                <>
                  {/* Chat Header */}
                  <Box sx={{ p: 2, borderBottom: 1, borderColor: 'divider' }}>
                    <Stack direction="row" spacing={2} alignItems="center">
                      <Avatar>
                        {selectedRoom.type === 'community' ? <GroupsIcon /> : <PersonIcon />}
                      </Avatar>
                      <Box>
                        <Typography variant="h6" fontWeight="bold">
                          {selectedRoom.name}
                        </Typography>
                        <Stack direction="row" spacing={2} alignItems="center">
                          <Typography variant="body2" color="text.secondary">
                            {selectedRoom.participants} participants
                          </Typography>
                          <Chip
                            icon={<EncryptionIcon />}
                            label="Encrypted"
                            size="small"
                            color="success"
                            variant="outlined"
                          />
                        </Stack>
                      </Box>
                    </Stack>
                  </Box>

                  {/* Connection Status */}
                  {isConnecting && (
                    <Alert severity="info" sx={{ m: 2 }}>
                      <Stack direction="row" spacing={1} alignItems="center">
                        <CircularProgress size={16} />
                        <Typography variant="body2">
                          Establishing secure connection...
                        </Typography>
                      </Stack>
                    </Alert>
                  )}

                  {connectionError && (
                    <Alert severity="error" sx={{ m: 2 }}>
                      {connectionError}
                    </Alert>
                  )}

                  {/* Messages Area */}
                  <Box
                    sx={{
                      flexGrow: 1,
                      p: 2,
                      overflow: 'auto',
                      bgcolor: 'grey.50',
                    }}
                  >
                    {messages.map((message) => (
                      <MessageBubble
                        key={message.id}
                        message={message}
                        isOwn={message.senderId === user?.sub}
                      />
                    ))}
                    <div ref={messagesEndRef} />
                  </Box>

                  {/* Message Input */}
                  <Box sx={{ p: 2, borderTop: 1, borderColor: 'divider' }}>
                    <Stack direction="row" spacing={1} alignItems="flex-end">
                      <TextField
                        fullWidth
                        multiline
                        maxRows={4}
                        placeholder="Type your message..."
                        value={newMessage}
                        onChange={(e) => setNewMessage(e.target.value)}
                        onKeyPress={handleKeyPress}
                        disabled={isConnecting}
                      />
                      <IconButton
                        color="primary"
                        onClick={handleSendMessage}
                        disabled={!newMessage.trim() || isConnecting}
                        sx={{ mb: 0.5 }}
                      >
                        <SendIcon />
                      </IconButton>
                    </Stack>
                  </Box>
                </>
              ) : (
                /* No Room Selected */
                <Box
                  sx={{
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    justifyContent: 'center',
                    height: '100%',
                    textAlign: 'center',
                  }}
                >
                  <ChatIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
                  <Typography variant="h6" color="text.secondary" gutterBottom>
                    Select a chat room to start messaging
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    Choose from community chats or direct messages
                  </Typography>
                </Box>
              )}
            </Paper>
          </Grid>
        </Grid>
      </Container>
    </DashboardLayout>
  );
}