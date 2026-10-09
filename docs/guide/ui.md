# Using the console

Unshift has a small, focused UI. Every screen is driven by the
[REST API](/api/) through a shared Pinia store, so what you see here is what
the API returns.

## Cluster overview (`/`)

The landing page shows the cluster at a glance:

- **Bootstrap servers** — the broker address the kernel connected with.
- **Total topics** and **total partitions**.
- **Preferred leader** — percentage of partitions led by their preferred broker.
- **Under-replicated** — partitions currently under-replicated.

Below the cards is a filterable **topics table** with columns for partitions,
preferred-leader percentage, under-replicated count and custom-config count. Use
**Create topic** to add a topic by name, partition count and replication factor.

## Topic detail (`/topics/{name}`)

Clicking a topic opens its detail page:

- Summary cards — partitions, preferred leader %, under-replicated, custom
  configs.
- A table of the topic's **custom configs** (name, value, whether it is
  read-only).
- Actions to **View messages** or **Publish messages** for that topic.

A topic sidebar lists all topics so you can jump between them.

## Messages (`/topics/{name}/messages`)

Reads from the earliest offset across all partitions, up to `limit` messages
(default `10`, maximum `100`). Each row shows the partition, offset, key and
payload so you can confirm what is on the topic.

## Publish (`/topics/{name}/messages/publish`)

The feature that sets Unshift apart from a read-only viewer:

- **Key** — optional; leave empty for a keyless message.
- **Payload** — edited in a code editor with JSON/YAML syntax modes and a
  **Format** action that pretty-prints valid JSON.
- **Save** — stores the current key/payload as a reusable template in the local
  database.
- **Publish** — produces the message to the topic.

Saved templates for the topic appear in a collapsible **Saved messages** panel,
where a click loads one back into the form and the trash icon deletes it.

### Errors

Failures surface inline in red. The underlying API returns Kafka failures as
`502` and storage failures as `500`, each with a JSON `message` body (see
[Errors](/guide/architecture#errors)).
