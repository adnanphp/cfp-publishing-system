#!/bin/bash

echo "Updating main.rs to include TextRepository..."

# Create a sed command to add TextRepository
sed -i '
/let author_repo = database::repositories::AuthorRepository::new(pool.clone())/a\
        let text_repo = database::repositories::TextRepository::new(pool.clone());
' src/main.rs

# Also add it to app_data
sed -i '
/\.app_data(web::Data::new(author_repo))/a\
            .app_data(web::Data::new(text_repo))
' src/main.rs

echo "✅ TextRepository added to main.rs"
