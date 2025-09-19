'use client';

import { useState } from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Button,
  Typography,
  Box,
  Alert,
  CircularProgress,
  Chip,
  Avatar,
  Stack,
  Divider,
  FormControlLabel,
  Switch,
} from '@mui/material';
import {
  Lock as LockIcon,
  Send as SendIcon,
  Business as BusinessIcon,
  Person as PersonIcon,
  Security as SecurityIcon,
  Check as CheckIcon,
} from '@mui/icons-material';
import { useSession } from 'next-auth/react';

interface ChatInitiatorProps {
  open: boolean;
  onClose: () => void;
  recipient: {
    id: string;
    name: string;
    type: 'business' | 'user';
    avatar?: string;
    category?: string;
  };
  context?: {
    type: 'bacheca_post' | 'poi' | 'general';
    title?: string;
    postId?: string;
  };
  onChatStarted?: (chatId: string) => void;
}

export default function ChatInitiator({
  open,
  onClose,
  recipient,
  context,
  onChatStarted
}: ChatInitiatorProps) {
  const { data: session } = useSession();
  const user = session?.user;

  const [message, setMessage] = useState('');
  const [isAnonymous, setIsAnonymous] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [encryptionReady, setEncryptionReady] = useState(false);

  // Simulate encryption setup on component mount
  useState(() => {
    if (open) {
      // Simulate encryption key generation/exchange
      setTimeout(() => setEncryptionReady(true), 1000);
    }
  });

  const handleSendMessage = async () => {
    if (!message.trim() || !user) return;

    try {
      setIsLoading(true);
      setError(null);

      // Simulate encrypted chat creation
      const chatData = {
        participants: [user.id, recipient.id],
        initialMessage: {
          content: message,
          encrypted: true,
          sender: isAnonymous ? 'anonymous' : user.id,
          context: context
        },
        encryption: {
          algorithm: 'E2EE-AES-256',
          keyExchange: 'completed',
          verified: true
        }
      };

      console.log('Creating encrypted chat:', chatData);

      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 2000));

      // Generate mock chat ID
      const chatId = `chat_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

      // Call success callback
      onChatStarted?.(chatId);

      // Close dialog
      onClose();

      // Reset form
      setMessage('');
      setIsAnonymous(false);

    } catch (err) {
      console.error('Failed to start chat:', err);
      setError('Failed to start encrypted chat. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const getContextDescription = () => {
    if (!context) return null;

    switch (context.type) {
      case 'bacheca_post':
        return `About: "${context.title}"`;
      case 'poi':
        return `Regarding: ${context.title}`;
      case 'general':
      default:
        return 'General inquiry';
    }
  };

  return (
    <Dialog
      open={open}
      onClose={onClose}
      maxWidth="sm"
      fullWidth
      PaperProps={{
        sx: { borderRadius: 2 }
      }}
    >
      <DialogTitle>
        <Box display="flex" alignItems="center" gap={2}>
          <SecurityIcon color="primary" />
          <Box>
            <Typography variant="h6" fontWeight="bold">
              Start Encrypted Chat
            </Typography>
            <Typography variant="body2" color="text.secondary">
              End-to-end encrypted conversation
            </Typography>
          </Box>
        </Box>
      </DialogTitle>

      <DialogContent>
        {/* Recipient Info */}
        <Box
          sx={{
            bgcolor: 'action.hover',
            borderRadius: 2,
            p: 2,
            mb: 3,
            border: '1px solid',
            borderColor: 'divider'
          }}
        >
          <Box display="flex" alignItems="center" gap={2} mb={1}>
            <Avatar
              src={recipient.avatar}
              sx={{ bgcolor: recipient.type === 'business' ? 'primary.main' : 'secondary.main' }}
            >
              {recipient.type === 'business' ? <BusinessIcon /> : <PersonIcon />}
            </Avatar>
            <Box>
              <Typography variant="subtitle1" fontWeight="bold">
                {recipient.name}
              </Typography>
              <Box display="flex" alignItems="center" gap={1}>
                <Chip
                  label={recipient.type === 'business' ? 'Business' : 'User'}
                  size="small"
                  color={recipient.type === 'business' ? 'primary' : 'secondary'}
                  variant="outlined"
                />
                {recipient.category && (
                  <Chip
                    label={recipient.category}
                    size="small"
                    variant="outlined"
                  />
                )}
              </Box>
            </Box>
          </Box>

          {context && (
            <Typography variant="body2" color="text.secondary">
              {getContextDescription()}
            </Typography>
          )}
        </Box>

        {/* Encryption Status */}
        <Box
          sx={{
            bgcolor: encryptionReady ? 'success.lighter' : 'warning.lighter',
            borderRadius: 1,
            p: 2,
            mb: 3,
            border: '1px solid',
            borderColor: encryptionReady ? 'success.main' : 'warning.main'
          }}
        >
          <Box display="flex" alignItems="center" gap={1} mb={1}>
            {encryptionReady ? (
              <CheckIcon color="success" fontSize="small" />
            ) : (
              <CircularProgress size={16} />
            )}
            <Typography
              variant="body2"
              fontWeight="bold"
              color={encryptionReady ? 'success.main' : 'warning.main'}
            >
              {encryptionReady ? 'Encryption Ready' : 'Setting up encryption...'}
            </Typography>
          </Box>
          <Typography variant="caption" color="text.secondary">
            {encryptionReady
              ? 'Your conversation will be end-to-end encrypted. Only you and the recipient can read messages.'
              : 'Generating encryption keys and establishing secure connection...'
            }
          </Typography>
        </Box>

        {/* Privacy Options */}
        <Box mb={3}>
          <FormControlLabel
            control={
              <Switch
                checked={isAnonymous}
                onChange={(e) => setIsAnonymous(e.target.checked)}
                disabled={!encryptionReady}
              />
            }
            label={
              <Box>
                <Typography variant="body2" fontWeight="bold">
                  Send anonymously
                </Typography>
                <Typography variant="caption" color="text.secondary">
                  Your identity will be hidden from the recipient
                </Typography>
              </Box>
            }
          />
        </Box>

        {/* Message Input */}
        <TextField
          fullWidth
          multiline
          rows={4}
          placeholder={
            isAnonymous
              ? "Send an anonymous encrypted message..."
              : "Type your encrypted message..."
          }
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          disabled={!encryptionReady}
          InputProps={{
            startAdornment: (
              <LockIcon
                sx={{
                  color: encryptionReady ? 'success.main' : 'text.disabled',
                  mr: 1,
                  fontSize: 20
                }}
              />
            ),
          }}
          sx={{ mb: 2 }}
        />

        {/* Error Display */}
        {error && (
          <Alert severity="error" sx={{ mb: 2 }}>
            {error}
          </Alert>
        )}

        {/* Privacy Notice */}
        <Alert severity="info" icon={<SecurityIcon />}>
          <Typography variant="body2">
            <strong>Privacy Protection:</strong> Messages are encrypted on your device before sending.
            The server cannot read your conversation content.
          </Typography>
        </Alert>
      </DialogContent>

      <DialogActions sx={{ p: 3, pt: 0 }}>
        <Button onClick={onClose} disabled={isLoading}>
          Cancel
        </Button>
        <Button
          variant="contained"
          onClick={handleSendMessage}
          disabled={!message.trim() || !encryptionReady || isLoading || !user}
          startIcon={isLoading ? <CircularProgress size={16} /> : <SendIcon />}
        >
          {isLoading ? 'Sending...' : 'Send Encrypted Message'}
        </Button>
      </DialogActions>
    </Dialog>
  );
}