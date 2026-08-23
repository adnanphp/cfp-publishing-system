#!/bin/bash

FILE="src/infrastructure/database/repositories/member_repository.rs"

# Backup
cp "$FILE" "${FILE}.backup"

# Fix the INSERT query (around line 43)
sed -i '43,66c\
        let row = sqlx::query!(\
            r#"\
            INSERT INTO members (\
                name, organization, pseudonym, primary_email, recovery_email,\
                password_hash, verification_matrix, matrix_expiry, join_date,\
                street, city, state, country, postal_code, introduced_by\
            )\
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)\
            RETURNING member_id\
            "#,\
            member.name,\
            member.organization,\
            member.pseudonym,\
            member.primary_email,\
            member.recovery_email,\
            member.password_hash,\
            member.verification_matrix.as_ref().map(|v| v.to_string()),\
            member.matrix_expiry,\
            member.join_date,\
            member.address.as_ref().map(|a| a.street.clone()),\
            member.address.as_ref().map(|a| a.city.clone()),\
            member.address.as_ref().map(|a| a.state.clone()),\
            member.address.as_ref().map(|a| a.country.clone()),\
            member.address.as_ref().map(|a| a.postal_code.clone()),\
            member.introduced_by\
        )' "$FILE"

# Fix the SELECT query (around line 111)
sed -i '111,136c\
        let row = sqlx::query!(\
            r#"\
            SELECT \
                m.member_id, m.name, m.organization, m.pseudonym,\
                m.primary_email, m.recovery_email, m.password_hash,\
                m.verification_matrix, m.matrix_expiry, m.join_date,\
                m.street, m.city, m.state, m.country, m.postal_code,\
                m.introduced_by, m.created_at, m.updated_at,\
                array_agg(DISTINCT mp.phone_number) as phone_numbers,\
                array_agg(DISTINCT mi.interest) as interests\
            FROM members m\
            LEFT JOIN member_phone_numbers mp ON m.member_id = mp.member_id\
            LEFT JOIN member_interests mi ON m.member_id = mi.member_id\
            WHERE m.member_id = $1\
            GROUP BY m.member_id\
            "#,\
            member_id\
        )' "$FILE"

# Fix the UPDATE query (around line 199)
sed -i '199,232c\
        sqlx::query!(\
            r#"\
            UPDATE members SET\
                name = $1,\
                organization = $2,\
                pseudonym = $3,\
                primary_email = $4,\
                recovery_email = $5,\
                password_hash = $6,\
                verification_matrix = $7,\
                matrix_expiry = $8,\
                street = $9,\
                city = $10,\
                state = $11,\
                country = $12,\
                postal_code = $13,\
                introduced_by = $14,\
                updated_at = NOW()\
            WHERE member_id = $15\
            "#,\
            member.name,\
            member.organization,\
            member.pseudonym,\
            member.primary_email,\
            member.recovery_email,\
            member.password_hash,\
            member.verification_matrix.as_ref().map(|v| v.to_string()),\
            member.matrix_expiry,\
            member.address.as_ref().map(|a| a.street.clone()),\
            member.address.as_ref().map(|a| a.city.clone()),\
            member.address.as_ref().map(|a| a.state.clone()),\
            member.address.as_ref().map(|a| a.country.clone()),\
            member.address.as_ref().map(|a| a.postal_code.clone()),\
            member.introduced_by,\
            member.member_id\
        )' "$FILE"

# Fix the downloads count query (around line 343)
sed -i '343,350c\
        let row = sqlx::query!(\
            r#"\
            SELECT COUNT(*) as count\
            FROM downloads\
            WHERE member_id = $1 AND download_date >= CURRENT_DATE - INTERVAL \'7 days\'\
            "#,\
            member_id\
        )' "$FILE"

# Fix the donations query (around line 358)
sed -i '358,366c\
        let row = sqlx::query!(\
            r#"\
            SELECT EXISTS(\
                SELECT 1 FROM donations WHERE member_id = $1\
            ) as "exists!"\
            "#,\
            member_id\
        )' "$FILE"

echo "Member repository queries updated to match schema"
