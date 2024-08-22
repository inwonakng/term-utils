UTIL_DIR=$HOME/.term-utils/
mkdir -p $UTIL_DIR

# cargo build --release

for file in target/release/*; do
  if [ -f "$file" ] && [ -x "$file" ]; then
    cp $file $UTIL_DIR
  fi
done

# check PATH
SHELL_CONFIG_FILES=("$HOME/.bashrc" "$HOME/.zshrc")

# Function to add UTIL_DIR to PATH if not already present
add_to_path() {
  local shell_config_file=$1

  if [ -f "$shell_config_file" ]; then
    if ! grep -q ".term-utils" "$shell_config_file"; then
      echo "Adding $UTIL_DIR to PATH in $shell_config_file"
      echo "export PATH=\$PATH:\"\$HOME/.term-utils\"" >>"$shell_config_file"
    else
      echo "$UTIL_DIR is already in PATH in $shell_config_file"
    fi
  fi
}

# Update each shell config file
for config_file in "${SHELL_CONFIG_FILES[@]}"; do
  add_to_path "$config_file"
done

echo "Please restart your shell for the changes to take affect."
