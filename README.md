# Docker GUI
## Development
Prepare the repo:
```sh
npm i
```

Run dev:
```sh
npm run dev
```

Build:

```sh
npm run build
```

## ToDo
### Frontend
- [ ] State Management
- [ ] GUI
- [ ] a11y
- [ ] i18n

### Backend
- [ ] Wrapper for Docker
- [ ] Emitters
- [ ] Functions

## API
### Frontend
### Backend
- fn `containerAll()` Returns all Docker Containers.
- fn `containerGet(container_id)` Returns Container informations by ID.
- fn `containerStop(container_id)` Stops Container.
- fn `containerRestart(container_id)` Restarts Container.
- fn `containerRemove(container_id)` Removes Container.

- fn `imagesAll()` Returns all Docker Images.
- fn `imagesGet(container_id)` Returns Image informations by ID.
- fn `imagesRemove(container_id)` Removes Image.

- listener `listen('logs', 'CONTAINER_ID')` - Will subsribe to a containers log.
- listener `listen('stats', 'CONTAINER_ID')` - Will subscribe to a containers stats.
- listener `listen('unsubscribe', 'logs|stats')` - Unsubscribe (There will only be a single sub to each channel)

- emitter `emit('logs', 'A LINE OF LOG'` - If subscribed - send every log to Frontend.
- emitter `emit('stats', 'Stats of the container'` - If subscribed - send stats every X seconds to Frontend.
