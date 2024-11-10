import { invoke } from "@tauri-apps/api/core";

class QueryService {
    async executeQuery(dataProviderId: string, sqlQuery: string): Promise<any[]> {
        try {
            console.log("executing query", dataProviderId, sqlQuery);
            const result: any[] = await invoke("execute_query", {
                dataProviderId: dataProviderId,
                sqlQuery: sqlQuery,
            });
            return result;
        } catch (error) {
            console.error("Error executing query", error);
            return [];
        }
    }
}

function createQueryService() {
    return new QueryService();
}

export default createQueryService;
