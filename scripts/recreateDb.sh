#!/bin/bash
set -e

rm /var/lib/cosmic/cosmic.db
touch /var/lib/cosmic/cosmic.db
echo "Database recreated successfully!"