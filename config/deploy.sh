
SPACER="======================================"
EG="🔷"

cd /dozar/
export $(head -n 1 secrets.env | xargs)

OLD_COMMIT=$(git rev-parse HEAD)

echo "$EG update the source"
git pull
echo $SPACER

NEW_COMMIT=$(git rev-parse HEAD)

function check_diff {
    local file_has_changed=$(git diff --name-only $OLD_COMMIT...$NEW_COMMIT --exit-code $1)
    if [ -z "$file_has_changed" ]; then
        return 1
    else
        return 0
    fi
}

if check_diff "config/dozar.service"; then
    echo "$EG reload the service"
    cp config/dozar.service /etc/systemd/system/ --force
    systemctl daemon-reload
    echo $SPACER
fi

if [ ! -f db/main.db ]; then
    echo "$EG setup the database"
    mkdir -p db
    cargo sqlx database setup
    echo $SPACER
fi

if check_diff "migrations/*"; then
    echo "$EG update the database"
    cargo sqlx database setup
    echo $SPACER
fi

if check_diff "package.json"; then
    echo "$EG install npm packages"
    npm i
    echo $SPACER
fi

if check_diff "app/* vite.ts"; then
    echo "$EG build the app!"
    npm run build
    echo $SPACER
fi

if check_diff "src/* Cargo.toml"; then
    echo "$EG cargo build"
    cargo build --release
    echo $SPACER
fi

echo "$EG restart dozar"
systemctl restart dozar
echo $SPACER

echo "$EG status dozar"
systemctl status dozar
echo $SPACER

echo "Deploy is Done! ✅"

