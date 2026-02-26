#!/bin/bash

# Agent Editor Helper Script
# Provides utilities for managing Qwen subagent configurations

set -e

AGENT_DIR="${PWD}/agent-design"
SKILL_DIR="${PWD}/.qwen/skills/agent-editor"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

echo_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

echo_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Validate agent file format
validate_agent() {
    local file=$1
    
    if [[ ! -f "$file" ]]; then
        echo_error "File not found: $file"
        return 1
    fi
    
    # Check for YAML frontmatter
    if ! head -20 "$file" | grep -q "^---$"; then
        echo_error "Missing YAML frontmatter in $file"
        return 1
    fi
    
    # Check for name field
    if ! grep -q "^name:" "$file"; then
        echo_error "Missing 'name' field in $file"
        return 1
    fi
    
    # Check for description field
    if ! grep -q "^description:" "$file"; then
        echo_error "Missing 'description' field in $file"
        return 1
    fi
    
    echo_success "Validated: $file"
    return 0
}

# List all agents
list_agents() {
    echo_info "Available agents in $AGENT_DIR:"
    echo ""
    
    for file in "$AGENT_DIR"/*.md; do
        if [[ -f "$file" ]]; then
            local name=$(grep "^name:" "$file" | sed 's/name: //')
            local desc=$(grep "^description:" "$file" | sed 's/description: //')
            printf "  %-25s %s\n" "$name" "$desc"
        fi
    done
}

# Check for consistency across agents
check_consistency() {
    echo_info "Checking agent consistency..."
    
    local coordinator_count=0
    local primary_found=false
    local secondary_found=false
    
    for file in "$AGENT_DIR"/*.md; do
        if [[ -f "$file" ]]; then
            # Check for project-manager references
            if grep -q "project-manager" "$file" && [[ "$file" != *project-manager.md ]]; then
                coordinator_count=$((coordinator_count + 1))
            fi
            
            # Check for PRIMARY POINT OF CONTACT
            if grep -q "PRIMARY POINT OF CONTACT" "$file"; then
                if [[ "$file" == *project-manager.md ]]; then
                    primary_found=true
                else
                    echo_error "PRIMARY POINT OF CONTACT found in $file (should only be in project-manager)"
                fi
            fi
            
            # Check for SECOND POINT OF CONTACT
            if grep -q "SECOND POINT OF CONTACT" "$file"; then
                if [[ "$file" == *architect-designer.md ]]; then
                    secondary_found=true
                else
                    echo_error "SECOND POINT OF CONTACT found in $file (should only be in architect-designer)"
                fi
            fi
            
            # Validate format
            validate_agent "$file" || return 1
        fi
    done
    
    echo_success "Found $coordinator_count agents referencing project-manager"
    
    if $primary_found; then
        echo_success "PRIMARY POINT OF CONTACT correctly set in project-manager"
    else
        echo_error "PRIMARY POINT OF CONTACT not found in project-manager"
        return 1
    fi
    
    if $secondary_found; then
        echo_success "SECOND POINT OF CONTACT correctly set in architect-designer"
    else
        echo_error "SECOND POINT OF CONTACT not found in architect-designer"
        return 1
    fi
    
    return 0
}

# Update all agents with a specific pattern
update_pattern() {
    local old_pattern=$1
    local new_pattern=$2
    
    echo_info "Replacing '$old_pattern' with '$new_pattern' in all agents..."
    
    for file in "$AGENT_DIR"/*.md; do
        if [[ -f "$file" ]]; then
            if grep -q "$old_pattern" "$file"; then
                local count=$(grep -c "$old_pattern" "$file")
                sed -i "s|$old_pattern|$new_pattern|g" "$file"
                echo_success "Updated $file ($count occurrences)"
            fi
        fi
    done
}

# Show agent statistics
show_stats() {
    echo_info "Agent Statistics:"
    echo ""
    
    local total_files=0
    local total_lines=0
    
    for file in "$AGENT_DIR"/*.md; do
        if [[ -f "$file" ]]; then
            local lines=$(wc -l < "$file")
            local name=$(basename "$file")
            printf "  %-30s %5d lines\n" "$name" "$lines"
            
            total_files=$((total_files + 1))
            total_lines=$((total_lines + lines))
        fi
    done
    
    echo ""
    echo "Total: $total_files agents, $total_lines lines"
}

# Main menu
show_help() {
    cat << EOF
Agent Editor Helper Script

Usage: $0 <command> [options]

Commands:
  list              List all available agents
  validate <file>   Validate a specific agent file
  validate-all      Validate all agent files
  check-consistency Check consistency across all agents
  update-pattern    Replace pattern in all agents
  stats             Show agent statistics
  help              Show this help message

Examples:
  $0 list
  $0 validate project-manager.md
  $0 check-consistency
  $0 update-pattern "old-text" "new-text"
  $0 stats
EOF
}

# Main execution
case "${1:-help}" in
    list)
        list_agents
        ;;
    validate)
        if [[ -z "$2" ]]; then
            echo_error "Usage: $0 validate <file>"
            exit 1
        fi
        validate_agent "$AGENT_DIR/$2"
        ;;
    validate-all)
        for file in "$AGENT_DIR"/*.md; do
            validate_agent "$file" || exit 1
        done
        echo_success "All agents validated successfully"
        ;;
    check-consistency)
        check_consistency
        ;;
    update-pattern)
        if [[ -z "$2" ]] || [[ -z "$3" ]]; then
            echo_error "Usage: $0 update-pattern <old-pattern> <new-pattern>"
            exit 1
        fi
        update_pattern "$2" "$3"
        ;;
    stats)
        show_stats
        ;;
    help|*)
        show_help
        ;;
esac
