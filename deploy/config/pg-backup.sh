#!/bin/bash
# PostgreSQL daily backup script for Civiqo Community Manager
# Run via cron: 0 3 * * * /opt/community-manager/config/pg-backup.sh

set -euo pipefail

BACKUP_DIR="/opt/community-manager/data/backups"
DAILY_DIR="$BACKUP_DIR/daily"
WEEKLY_DIR="$BACKUP_DIR/weekly"
DB_NAME="community_manager"
DATE=$(date +%Y-%m-%d_%H%M)
DAY_OF_WEEK=$(date +%u)

mkdir -p "$DAILY_DIR" "$WEEKLY_DIR"

# Daily backup
echo "[$(date)] Starting daily backup..."
pg_dump -U community_manager "$DB_NAME" | gzip > "$DAILY_DIR/backup_${DATE}.sql.gz"
echo "[$(date)] Daily backup completed: backup_${DATE}.sql.gz"

# Weekly backup on Sunday (day 7)
if [ "$DAY_OF_WEEK" -eq 7 ]; then
    cp "$DAILY_DIR/backup_${DATE}.sql.gz" "$WEEKLY_DIR/weekly_${DATE}.sql.gz"
    echo "[$(date)] Weekly backup saved"
fi

# Retention: keep 7 daily, 4 weekly
find "$DAILY_DIR" -name "backup_*.sql.gz" -mtime +7 -delete 2>/dev/null || true
find "$WEEKLY_DIR" -name "weekly_*.sql.gz" -mtime +28 -delete 2>/dev/null || true

echo "[$(date)] Backup cleanup complete"
