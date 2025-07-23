
----

## Implementation

### Step-by-Step Plan

#### 1. `mindvault-api`
- [ ] Set up Axum project with MongoDB integration.
- [ ] Define `Comment` model with JSON (de)serialization.
- [ ] Implement routes:
    - `POST /comments` → add comment
    - `GET /comments` → query by `person_name`, date range, or keyword
- [ ] Add input validation and error handling
- [ ] Containerize with Docker (optional for deployability)

#### 2. `mindvault-tool`
- [ ] Set up Rust project and add `rust-mcp` SDK
- [ ] Define tool: `summarize_performance(name: String, time_range: String)`
- [ ] Call API with extracted name/time
- [ ] Receive and format comment history
- [ ] Return markdown-style summary input back to Claude

#### 3. `frontend`
- [ ] Set up Dixous project
- [ ] Build pages:
    - Comment input form
    - Person history viewer with filters (time, keyword)
- [ ] Connect to `mindvault-api`
- [ ] Optional: Integrate with Claude to show summaries in browser

#### 4. Shared Library (`common`)
- [ ] Define `Comment` struct with shared types
- [ ] Utility helpers for formatting, validation

---

## Milestones

### Phase 1: Core API & Tool
- [ ] Initialize `mindvault-api` project with MongoDB connection
- [ ] Implement `/comments` POST and GET endpoints
- [ ] Build `mindvault-tool` project and define tool interface
- [ ] Test Claude integration using mock data

### Phase 2: Frontend UI
- [ ] Setup Dixous frontend app with dev server
- [ ] Implement form for adding comments
- [ ] Implement filter UI for viewing feedback by name and time range
- [ ] Connect frontend to `mindvault-api`

### Phase 3: Polishing & Deployment
- [ ] Add keyword inference via Claude (optional enhancement)
- [ ] Containerize backend API and frontend for deployment
- [ ] Prepare Claude prompt templates and tool manifest
- [ ] Write documentation and usage examples

---

## Gathering Results

To evaluate the success and effectiveness of MindVault, the following criteria will be tracked post-deployment:

### Functionality Validation
- [ ] Comments can be added and retrieved with accurate filters.
- [ ] Claude or LM Studio correctly receives and interprets tool outputs.
- [ ] Summaries generated reflect meaningful aggregation of comments.

### Usability Metrics
- [ ] Time taken by leads to add or retrieve feedback is low.
- [ ] Feedback coverage across all team members is consistent.
- [ ] Frontend and tool interaction is seamless and intuitive.

### System Performance
- [ ] MongoDB query performance under expected load (filtering large comment sets).
- [ ] API uptime and latency benchmarks.
- [ ] Tool latency (Claude ↔ API ↔ DB) for summary generation.

### LLM Feedback Quality (Qualitative)
- [ ] Lead satisfaction with generated summaries.
- [ ] Degree to which summaries help in performance reviews or retros.

### Future Enhancements
- Incorporate automatic keyword extraction.
- Allow feedback editing and versioning.
- Add sentiment scoring per comment.

---