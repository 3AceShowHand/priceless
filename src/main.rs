use std::fmt;

#[derive(Debug, Clone)]
struct Stock {
    company: Company,
    price: f64,
    market: Market,
    basic_info: BasicInfo,
}

impl Stock {
    fn new(company: Company, price: f64, market: Market, basic_info: BasicInfo) -> Stock {
        Stock {
            company,
            price,
            market,
            basic_info,
        }
    }

    fn get_valuation(&self, model: ValuationModel) -> f64 {
        match model {
            ValuationModel::DiscountedCashflow => self.discounted_cashflow(),
            ValuationModel::PeterLynch => self.peter_lynch(),
            ValuationModel::BenjaminGraham => self.benjamin_graham(),
            ValuationModel::TangChao => self.tang_chao(),
        }
    }

    fn discounted_cashflow(&self) -> f64 {
        0.0
    }

    fn peter_lynch(&self) -> f64 {
        0.0
    }

    fn benjamin_graham(&self) -> f64 {
        0.0
    }

    fn tang_chao(&self) -> f64 {
        0.0
    }
}

#[derive(Debug, Clone)]
struct BasicInfo {
    name: String,
    code: String,
    industry: String,
    area: String,
    pe: f64,
    pb: f64,
    outstanding: f64,
    totals: f64,
    total_assets: f64,
    liquid_assets: f64,
    fixed_assets: f64,
    reserved: f64,
    reserved_per_share: f64,
    esp: f64,
    bvps: f64,
    time_to_market: String,
}

impl BasicInfo {
    fn new(
        name: String,
        code: String,
        industry: String,
        area: String,
        pe: f64,
        pb: f64,
        outstanding: f64,
        totals: f64,
        total_assets: f64,
        liquid_assets: f64,
        fixed_assets: f64,
        reserved: f64,
        reserved_per_share: f64,
        esp: f64,
        bvps: f64,
        time_to_market: String,
    ) -> BasicInfo {
        BasicInfo {
            name,
            code,
            industry,
            area,
            pe,
            pb,
            outstanding,
            totals,
            total_assets,
            liquid_assets,
            fixed_assets,
            reserved,
            reserved_per_share,
            esp,
            bvps,
            time_to_market,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Company {
    name: String,
    code: String,
}

impl Company {
    fn new(name: String, code: String) -> Company {
        Company { name, code }
    }
}

#[derive(Debug, Clone)]
enum Market {
    Shanghai,
    Shenzhen,
    Beijing,
    ChuangYe,
    KeChuang,
}

impl fmt::Debug for Market {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Market::Shanghai => "Shanghai".fmt(f),
            Market::Shenzhen => "Shenzhen".fmt(f),
            Market::Beijing => "Beijing".fmt(f),
            Market::ChuangYe => "ChuangYe".fmt(f),
            Market::KeChuang => "KeChuang".fmt(f),
        }
    }
}

enum ValuationModel {
    DiscountedCashflow,
    PeterLynch,
    BenjaminGraham,
    TangChao,
}

impl fmt::Debug for ValuationModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValuationModel::DiscountedCashflow => "DiscountedCashflow".fmt(f),
            ValuationModel::PeterLynch => "PeterLynch".fmt(f),
            ValuationModel::BenjaminGraham => "BenjaminGraham".fmt(f),
            ValuationModel::TangChao => "TangChao".fmt(f),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
