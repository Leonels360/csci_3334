#!/bin/bash

mkdir -p books
cd books

echo "Downloading books from Project Gutenberg..."
echo "This will a take a wihle"
echo ""

count=0
id=1

while [ $count -lt 100 ]; do
    wget -q -O "book_${id}.txt" "https://www.gutenberg.org/cache/epub/${id}/pg${id}.txt" 2>/dev/null
    
    if [ -s "book_${id}.txt" ]; then
        count=$((count+1))
        if [ $((count % 10)) -eq 0 ]; then
            echo "✓ Downloaded $count/100 books..."
        fi
    else
        rm -f "book_${id}.txt"
    fi
    
    id=$((id+1))
    sleep 0.2
done

echo ""
echo "✓ Done! Downloaded $count books to ./books/"
