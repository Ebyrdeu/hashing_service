## Salted Hash API

### Overview

This API is designed to provide secure hashing functionalities with both manual
and automatic salt generation for SHA256 and SHA512 hash functions. It allows users to generate salted hashes and
compare given hashes with plaintext passwords.

### API Endpoints

#### SHA256 Endpoints

- **POST `/sha256/method/manual`**
    - **Description**: Generates a SHA256 hash with a user-provided salt.
    - **Request Body**:
      ```json
      {
        "password": "yourpassword",
        "rounds": 10,
        "salt": "yoursalt"
      }
      ```
    - **Response**:
      ```json
      {
        "password": "hashed_password"
      }
      ```

- **POST `/sha256/method/auto`**
    - **Description**: Generates a SHA256 hash with an automatically generated salt.
    - **Request Body**:
      ```json
      {
        "password": "yourpassword",
        "rounds": 10
      }
      ```
    - **Response**:
      ```json
      {
        "password": "hashed_password",
        "salt": "auto_generated_salt"
      }
      ```

- **POST `/sha256/:hashed-password`**
    - **Description**: Compares a provided password with a stored hash using SHA256.
    - **Request Body**:
      ```json
      {
        "password": "yourpassword",
        "rounds": 10,
        "salt": "used_salt"
      }
      ```
    - **Response**:
      ```json
      {
        "is_equal": true
      }
      ```

#### SHA512 Endpoints

- **POST `/sha512/method/manual`**
    - **Description**: Same as SHA256 but uses the SHA512 hash function.

- **POST `/sha512/method/auto`**
    - **Description**: Same as SHA256 but uses the SHA512 hash function.

- **POST `/sha512/:hashed-password`**
    - **Description**: Same as SHA256 but uses the SHA512 hash function.