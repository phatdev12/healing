export const EnvConfig = {
    pocketbaseUrl: process.env.POCKETBASE_URL,
    pocketbaseAdminEmail: process.env.POCKETBASE_ADMIN_EMAIL || "",
    pocketbaseAdminPassword: process.env.POCKETBASE_ADMIN_PASSWORD || "",
    PORT: process.env.PORT || 3000
}