<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { closeBrackets, closeBracketsKeymap } from '@codemirror/autocomplete'
import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands'
import { json } from '@codemirror/lang-json'
import { yaml } from '@codemirror/lang-yaml'
import { bracketMatching, HighlightStyle, indentOnInput, indentUnit, syntaxHighlighting } from '@codemirror/language'
import { Compartment, EditorState, type Extension } from '@codemirror/state'
import { drawSelection, EditorView, highlightActiveLine, highlightActiveLineGutter, keymap, lineNumbers, placeholder as placeholderExtension } from '@codemirror/view'
import { tags } from '@lezer/highlight'

export type EditorLanguage = 'auto' | 'json' | 'yaml'

const props = withDefaults(defineProps<{
  modelValue: string
  language?: EditorLanguage
  placeholder?: string
}>(), {
  language: 'auto',
  placeholder: ''
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const container = ref<HTMLElement | null>(null)

let view: EditorView | null = null
let suppressSync = false

const languageCompartment = new Compartment()

function detectLanguage(doc: string): 'json' | 'yaml' {
  const trimmed = doc.trim()

  if (trimmed.startsWith('{') || trimmed.startsWith('[')) {
    try {
      JSON.parse(trimmed)
      return 'json'
    } catch {
      return 'yaml'
    }
  }

  return 'yaml'
}

function extensionFor(language: 'auto' | 'json' | 'yaml'): Extension {
  return language === 'json' ? json() : yaml()
}

let detectedLanguage: 'json' | 'yaml' = 'yaml'

function applyLanguage(doc: string) {
  if (!view) {
    return
  }

  const current = detectLanguage(doc)

  if (current !== detectedLanguage) {
    detectedLanguage = current
    view.dispatch({ effects: languageCompartment.reconfigure(extensionFor(current)) })
  }
}

const highlightStyle = HighlightStyle.define([
  { tag: [tags.keyword, tags.modifier], color: 'var(--code-keyword)' },
  { tag: [tags.string, tags.special(tags.string)], color: 'var(--code-string)' },
  { tag: [tags.number, tags.integer, tags.float], color: 'var(--code-number)' },
  { tag: [tags.bool], color: 'var(--code-bool)' },
  { tag: [tags.null, tags.atom], color: 'var(--code-atom)' },
  { tag: [tags.propertyName, tags.attributeName], color: 'var(--code-property)' },
  { tag: [tags.comment], color: 'var(--code-comment)', fontStyle: 'italic' },
  { tag: [tags.operator, tags.punctuation], color: 'var(--code-operator)' }
])

const editorTheme = EditorView.theme({
  '&': {
    height: '100%',
    fontSize: '13px',
    lineHeight: '1.6',
    backgroundColor: 'transparent',
    color: 'var(--code-text)',
    border: '1px solid var(--code-border)',
    borderRadius: '0.75rem',
    overflow: 'hidden'
  },
  '&.cm-focused': {
    borderColor: 'var(--code-focus)'
  },
  '.cm-scroller': {
    overflow: 'auto',
    fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace'
  },
  '.cm-content': {
    padding: '0.75rem 0'
  },
  '.cm-cursor, .cm-dropCursor': {
    borderLeftColor: 'var(--code-caret)'
  },
  '.cm-gutters': {
    backgroundColor: 'transparent',
    color: 'var(--code-gutter)',
    borderRight: '1px solid var(--code-border)'
  },
  '.cm-activeLine': {
    backgroundColor: 'var(--code-active-line)'
  },
  '.cm-activeLineGutter': {
    backgroundColor: 'var(--code-active-line)'
  },
  '.cm-selectionBackground, .cm-content ::selection': {
    backgroundColor: 'var(--code-selection)'
  },
  '.cm-placeholder': {
    color: 'var(--code-gutter)'
  }
})

const extensions: Extension[] = [
  lineNumbers(),
  highlightActiveLineGutter(),
  highlightActiveLine(),
  drawSelection(),
  history(),
  closeBrackets(),
  bracketMatching(),
  indentOnInput(),
  indentUnit.of('  '),
  keymap.of([...closeBracketsKeymap, ...defaultKeymap, ...historyKeymap, indentWithTab]),
  languageCompartment.of(extensionFor(props.language)),
  placeholderExtension(props.placeholder),
  syntaxHighlighting(highlightStyle),
  editorTheme,
  EditorView.contentAttributes.of({
    spellcheck: 'false',
    autocapitalize: 'off',
    autocomplete: 'off'
  }),
  EditorView.updateListener.of((update) => {
    if (update.docChanged) {
      if (!suppressSync) {
        emit('update:modelValue', update.state.doc.toString())
      }

      if (props.language === 'auto') {
        applyLanguage(update.state.doc.toString())
      }
    }
  })
]

function syncDocFromModel(value: string) {
  if (!view || value === view.state.doc.toString()) {
    return
  }

  suppressSync = true
  view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: value } })
  suppressSync = false
}

watch(() => props.modelValue, syncDocFromModel)

watch(() => props.language, () => {
  if (!view) {
    return
  }

  detectedLanguage = props.language === 'auto' ? detectLanguage(view.state.doc.toString()) : props.language
  view.dispatch({ effects: languageCompartment.reconfigure(extensionFor(props.language)) })
})

onMounted(() => {
  if (!container.value) {
    return
  }

  detectedLanguage = detectLanguage(props.modelValue)

  const state = EditorState.create({
    doc: props.modelValue,
    extensions
  })

  view = new EditorView({ state, parent: container.value })
  view.focus()
})

onBeforeUnmount(() => {
  view?.destroy()
  view = null
})
</script>

<template>
  <div
    ref="container"
    class="h-full w-full overflow-hidden"
  />
</template>

<style>
:root {
  --code-text: #24292f;
  --code-keyword: #cf222e;
  --code-string: #0a3069;
  --code-number: #0550ae;
  --code-bool: #cf222e;
  --code-atom: #0550ae;
  --code-property: #953800;
  --code-comment: #6e7781;
  --code-operator: #24292f;
  --code-punctuation: #24292f;
  --code-caret: #24292f;
  --code-gutter: #9ca3af;
  --code-border: #d1d5db;
  --code-focus: #000000;
  --code-active-line: rgb(0 0 0 / 0.04);
  --code-selection: rgb(100 116 139 / 0.22);
}

.dark {
  --code-text: #e6edf3;
  --code-keyword: #ff7b72;
  --code-string: #a5d6ff;
  --code-number: #79c0ff;
  --code-bool: #ff7b72;
  --code-atom: #79c0ff;
  --code-property: #d2a8ff;
  --code-comment: #8b949e;
  --code-operator: #e6edf3;
  --code-punctuation: #e6edf3;
  --code-caret: #e6edf3;
  --code-gutter: #6b7280;
  --code-border: #4b5563;
  --code-focus: #9ca3af;
  --code-active-line: rgb(255 255 255 / 0.05);
  --code-selection: rgb(147 197 253 / 0.25);
}
</style>
