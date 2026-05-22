#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map};

const TOTAL_FUNDS: soroban_sdk::Symbol = symbol_short!("Total");
const SHARES: soroban_sdk::Symbol = symbol_short!("Shares");
const REVENUE: soroban_sdk::Symbol = symbol_short!("Revenue");

#[contract]
pub struct RevenueShareContract;

#[contractimpl]
impl RevenueShareContract {
    
    // 1. Hàm khởi tạo quỹ
    // 2. Hàm nạp tiền góp quỹ (Giai đoạn chuẩn bị dự án)
    pub fn contribute(env: Env, contributor: Address, amount: i128) {
        contributor.require_auth();

        let mut shares: Map<Address, i128> = env.storage().instance().get(&SHARES).unwrap();
        let current_share = shares.get(contributor.clone()).unwrap_or(0);
        
        shares.set(contributor, current_share + amount);
        env.storage().instance().set(&SHARES, &shares);

        let total: i128 = env.storage().instance().get(&TOTAL_FUNDS).unwrap();
        env.storage().instance().set(&TOTAL_FUNDS, &(total + amount));
    }

    // 3. Hàm cập nhật tiền thưởng/doanh thu đổ về (Giai đoạn đoạt giải)
    pub fn update_revenue(env: Env, amount: i128) {
        // Hàm này giả định tiền thưởng đã được chuyển thành công vào ví Contract
        let current_revenue: i128 = env.storage().instance().get(&REVENUE).unwrap_or(0);
        env.storage().instance().set(&REVENUE, &(current_revenue + amount));
    }

    // 4. BẬC THẦY MVP: Hàm tính toán và rút tiền thưởng theo tỷ lệ % đóng góp
    pub fn claim_reward(env: Env, contributor: Address) -> i128 {
        contributor.require_auth();

        let shares: Map<Address, i128> = env.storage().instance().get(&SHARES).unwrap();
        let my_contribution = shares.get(contributor.clone()).unwrap_or(0);
        
        let total_contributed: i128 = env.storage().instance().get(&TOTAL_FUNDS).unwrap();
        let total_revenue: i128 = env.storage().instance().get(&REVENUE).unwrap();

        // Kiểm tra xem người này có đóng góp không và hệ thống đã có tiền thưởng chưa
        if my_contribution == 0 || total_contributed == 0 || total_revenue == 0 {
            return 0;
        }

        // Thuật toán chia tỷ lệ: (Tiền góp của tôi * Tổng tiền thưởng) / Tổng quỹ đóng góp
        let my_reward = (my_contribution * total_revenue) / total_contributed;

        /*
         * Thực tế ở đây sẽ có lệnh: token.transfer(current_contract, contributor, my_reward)
         * Để giữ MVP đơn giản nhất, ta trả về số tiền mà họ xứng đáng được nhận để test logic.
         */
        my_reward
    }

    // Read-only preview function: compute reward without requiring auth
    pub fn preview_reward(env: Env, contributor: Address) -> i128 {
        let shares: Map<Address, i128> = env.storage().instance().get(&SHARES).unwrap_or(Map::new(&env));
        let my_contribution = shares.get(contributor.clone()).unwrap_or(0);

        let total_contributed: i128 = env.storage().instance().get(&TOTAL_FUNDS).unwrap_or(0);
        let total_revenue: i128 = env.storage().instance().get(&REVENUE).unwrap_or(0);

        if my_contribution == 0 || total_contributed == 0 || total_revenue == 0 {
            return 0;
        }

        (my_contribution * total_revenue) / total_contributed
    }
}