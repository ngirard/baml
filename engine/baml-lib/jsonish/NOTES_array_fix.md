# Notes on array coercion bug fix

## Background

BAML uses the `jsonish` crate to coerce loosely formatted JSON coming from an LLM into strongly typed values. Lists were previously accepted even when **all** of their items failed to parse. In that case the parser silently produced an empty array. This behaviour hid real problems: a list filled with malformed objects would become `[]` without any error.

A user reported that a valid JSON payload was parsed into a `QuestionGenerationResult` with an empty `potentialQuestions` list. Investigation showed that each question object contained an invalid enum value. The parser attempted to coerce each item, saw that none were valid and finally returned an empty list.

## Fix

`coerce_array` now tracks whether any items were present in the input. If parsing produced no valid items, `error_unexpected_empty_array` is returned. This ensures the caller receives a clear failure instead of an empty list.

A regression test reproduces the issue with a minimal `QuestionGenerationResult` schema. The failing case demonstrates that an invalid enum causes an error rather than silently dropping the item.

## Side effects

This change surfaced an unrelated failing test (`test_sidd`). That test includes Python snippets with square brackets that are interpreted as arrays. Because those arrays contain invalid elements, the new check triggered. After investigation we determined the extra arrays were not part of the intended JSON structure, so rejecting them is correct. The test has been updated accordingly.

