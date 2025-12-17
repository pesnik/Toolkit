/**
 * Tool Calling Utilities
 *
 * Utilities for detecting and parsing tool calls from LLM responses.
 */

import { ToolCall } from '@/types/ai-types';

/**
 * Detect if a message contains a tool call
 */
export function detectToolCall(content: string): boolean {
    return content.includes('<tool_call>') && content.includes('</tool_call>');
}

/**
 * Extract tool calls from LLM response
 * Supports XML-style tags: <tool_call>...</tool_call>
 */
export function extractToolCalls(content: string): ToolCall[] {
    const toolCalls: ToolCall[] = [];
    const regex = /<tool_call>([\s\S]*?)<\/tool_call>/g;
    let match;

    while ((match = regex.exec(content)) !== null) {
        try {
            let jsonContent = match[1].trim();

            console.log('[ToolCalling] Raw JSON content:', jsonContent);

            // Try to extract just the JSON object if there's extra text
            const jsonMatch = jsonContent.match(/\{[\s\S]*\}/);
            if (jsonMatch) {
                jsonContent = jsonMatch[0];
            }

            console.log('[ToolCalling] Cleaned JSON:', jsonContent);

            const parsed = JSON.parse(jsonContent);

            console.log('[ToolCalling] Parsed tool call:', parsed);

            // Validate required fields
            if (parsed.name && parsed.arguments) {
                toolCalls.push({
                    id: parsed.id || `call_${Date.now()}_${toolCalls.length}`,
                    name: parsed.name,
                    arguments: parsed.arguments,
                });
                console.log('[ToolCalling] ✅ Valid tool call added:', parsed.name);
            } else {
                console.warn('[ToolCalling] ⚠️ Tool call missing required fields:', parsed);
            }
        } catch (error) {
            console.error('[ToolCalling] ❌ Failed to parse tool call:', error);
            console.error('[ToolCalling] Content that failed:', match[1]);
        }
    }

    console.log(`[ToolCalling] Total tool calls extracted: ${toolCalls.length}`);
    return toolCalls;
}

/**
 * Remove tool call tags from content (for display)
 */
export function removeToolCallTags(content: string): string {
    return content.replace(/<tool_call>[\s\S]*?<\/tool_call>/g, '').trim();
}

/**
 * Format tool result for LLM
 */
export function formatToolResult(toolName: string, result: string, isError: boolean): string {
    if (isError) {
        return `<tool_result name="${toolName}" error="true">
${result}
</tool_result>`;
    }

    return `<tool_result name="${toolName}">
${result}
</tool_result>`;
}

/**
 * Check if content has tool result tags
 */
export function hasToolResult(content: string): boolean {
    return content.includes('<tool_result') && content.includes('</tool_result>');
}
