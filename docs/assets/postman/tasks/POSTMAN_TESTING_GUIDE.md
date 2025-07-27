# Postman Collection Testing Guide

## Overview

This guide explains how to use the Postman collection for testing the Task Management API. The collection includes comprehensive test scenarios, automated tests, and environment variables for easy testing.

## Files Included

1. **Task_Management_API.postman_collection.json** - Main collection with all endpoints
2. **Task_Management_Environment.postman_environment.json** - Environment variables
3. **POSTMAN_TESTING_GUIDE.md** - This guide

## Setup Instructions

### 1. Import Collection and Environment

1. Open Postman
2. Click **Import** button
3. Import both files:
   - `Task_Management_API.postman_collection.json`
   - `Task_Management_Environment.postman_environment.json`
4. Select the "Task Management Environment" from the environment dropdown

### 2. Configure Environment Variables

Update these variables in the environment as needed:

| Variable | Default Value | Description |
|----------|---------------|-------------|
| `base_url` | `http://localhost:8080` | API server URL |
| `task_id` | `1` | Task ID for single operations |
| `status` | `Pending` | Status filter for searches |
| `priority` | `High` | Priority filter for searches |
| `search_query` | `review` | Text search term |
| `due_date` | `2024-02-15` | Date filter for searches |

## Collection Structure

### 📁 Tasks
Basic CRUD operations for individual tasks:
- **GET** Get All Tasks
- **GET** Get Task by ID  
- **POST** Create Task
- **PUT** Update Task
- **DELETE** Delete Task (Soft Delete)

### 📁 Bulk Operations
Efficient bulk operations:
- **POST** Bulk Create Tasks
- **DELETE** Bulk Delete by Status

### 📁 Search Operations
Advanced search and filtering:
- **GET** Search Tasks by Text
- **GET** Search Tasks by Status
- **GET** Search Tasks by Priority
- **GET** Search Tasks by Due Date
- **GET** Complex Search (multiple filters)
- **PUT** Search and Update Tasks

## Testing Scenarios

### Scenario 1: Basic CRUD Workflow

1. **Create Task** - Creates a new task and saves ID
2. **Get All Tasks** - Verify task appears in list
3. **Get Task by ID** - Retrieve specific task
4. **Update Task** - Modify task status/priority
5. **Delete Task** - Soft delete the task
6. **Get Task by ID** - Verify task is no longer accessible

### Scenario 2: Bulk Operations

1. **Bulk Create Tasks** - Create multiple tasks at once
2. **Get All Tasks** - Verify all tasks were created
3. **Bulk Delete by Status** - Delete all tasks with specific status
4. **Get All Tasks** - Verify tasks were deleted

### Scenario 3: Search and Filter

1. **Search by Text** - Find tasks containing specific text
2. **Search by Status** - Filter tasks by status
3. **Search by Priority** - Filter tasks by priority
4. **Complex Search** - Combine multiple filters
5. **Search and Update** - Find and update matching tasks

### Scenario 4: Error Handling

1. **Get Non-existent Task** - Should return 404
2. **Update with No Fields** - Should return 400
3. **Delete Non-existent Task** - Should return 404
4. **Invalid Status in Bulk Delete** - Should return 400

## Automated Tests

The collection includes automated tests that verify:

### Response Validation
- Status codes (200, 400, 404, 500)
- Response structure and data types
- Required fields presence
- Response time performance

### Data Integrity
- Created tasks have correct properties
- Updates modify only specified fields
- Soft deletes don't physically remove records
- Search results match filter criteria

### Error Handling
- Proper error messages for invalid requests
- Correct status codes for different error types
- Validation of required fields

## Running Tests

### Individual Requests
1. Select any request from the collection
2. Click **Send**
3. View response and test results in the **Test Results** tab

### Collection Runner
1. Click on the collection name
2. Click **Run collection**
3. Select requests to run
4. Configure iterations and delays
5. Click **Run Task Management API**

### Newman (Command Line)
```bash
# Install Newman
npm install -g newman

# Run collection
newman run Task_Management_API.postman_collection.json \
  -e Task_Management_Environment.postman_environment.json \
  --reporters cli,html \
  --reporter-html-export results.html
```

## Environment Variables Usage

### Dynamic Variables
- `{{created_task_id}}` - Automatically set after creating a task
- `{{$timestamp}}` - Current timestamp
- `{{$randomInt}}` - Random integer

### Custom Variables
Update these based on your test data:
```json
{
  "task_id": "123",
  "status": "Completed",
  "priority": "Normal",
  "search_query": "urgent"
}
```

## Sample Request Bodies

### Create Task
```json
{
  "name": "Review code changes",
  "dueDate": "2024-02-15",
  "priority": "High",
  "status": "Pending"
}
```

### Update Task
```json
{
  "status": "Completed",
  "priority": "Normal"
}
```

### Bulk Create
```json
{
  "tasks": [
    {
      "name": "Task 1",
      "priority": "High",
      "status": "Pending"
    },
    {
      "name": "Task 2",
      "dueDate": "2024-02-20",
      "priority": "Normal"
    }
  ]
}
```

### Search and Update
```json
{
  "query": "review",
  "statusFilter": "Pending",
  "status": "InProgress",
  "priority": "High"
}
```

## Expected Responses

### Success Responses
- **200 OK** - Successful operation
- **201 Created** - Resource created (if applicable)

### Error Responses
- **400 Bad Request** - Invalid input or validation error
- **404 Not Found** - Resource not found
- **500 Internal Server Error** - Server error

## Troubleshooting

### Common Issues

1. **Connection Refused**
   - Ensure the API server is running on `localhost:8080`
   - Check the `base_url` environment variable

2. **404 Not Found**
   - Verify the task ID exists
   - Check if task was soft-deleted

3. **400 Bad Request**
   - Validate request body format
   - Ensure required fields are provided

4. **Test Failures**
   - Check response format matches expected structure
   - Verify environment variables are set correctly

### Debug Tips
- Use Postman Console (View → Show Postman Console) to see detailed logs
- Check the **Test Results** tab for specific test failures
- Verify environment variable values in the environment editor

## Best Practices

1. **Run tests in order** - Some tests depend on previous test results
2. **Use environment variables** - Makes tests reusable across environments
3. **Check test results** - Don't just look at status codes, verify test assertions
4. **Clean up test data** - Delete test tasks after testing if needed
5. **Use meaningful names** - Update task names to reflect test scenarios
