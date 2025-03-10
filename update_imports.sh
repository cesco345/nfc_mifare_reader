#!/bin/bash

# Script to update all inventory UI imports across the project

# Files we need to update
FILES=(
  "src/app/events.rs"
  "src/app/init.rs"
  "src/app/menu.rs"
  "src/db_viewer.rs"
  "src/main.rs"
  "src/reader/ui.rs"
  "src/sync/file_sync.rs"
  "src/sync/mod.rs"
)

# Find and replace in each file
for file in "${FILES[@]}"; do
  if [ -f "$file" ]; then
    echo "Updating $file..."
    
    # Replace import statements
    sed -i '' 's/use crate::inventory::ui::actions::InventoryUI;/use crate::inventory::InventoryUI;/g' "$file"
    sed -i '' 's/use crate::inventory::ui::actions;/use crate::inventory::InventoryUI;/g' "$file"
    
    # Replace type references in function signatures and variables
    sed -i '' 's/crate::inventory::ui::actions::InventoryUI/crate::inventory::InventoryUI/g' "$file"
    
    # Replace instantiation
    sed -i '' 's/inventory::ui::actions::InventoryUI::new/inventory::InventoryUI::new/g' "$file"
    
    echo "Done updating $file"
  else
    echo "Warning: File $file not found"
  fi
done

echo "All import updates completed!"