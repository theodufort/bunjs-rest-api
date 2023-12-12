CREATE TABLE AmdProcessorsTypes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  architecture TEXT,
  fabrication_nm TEXT,
  family TEXT,
  release_date TEXT,
  codename TEXT,
  model_group TEXT,
  cores TEXT,
  smt TEXT,
  clock_rate_mhz TEXT,
  bus_speed_type TEXT,
  cache_l1 TEXT,
  cache_l2 TEXT,
  cache_l3 TEXT,
  socket TEXT,
  memory_controller TEXT,
  simd TEXT,
  speed_power TEXT,
  other TEXT,
  changes TEXT
);

CREATE TABLE IntelProcessorsTypes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  processor TEXT,
  series_nomenclature TEXT,
  codename TEXT,
  production_date TEXT,
  clock_rate TEXT,
  socket TEXT,
  fabrication_nm TEXT,
  tdp TEXT,
  cores TEXT,
  bus_speed TEXT,
  cache_l1 TEXT,
  cache_l2 TEXT,
  cache_l3 TEXT,
  overclock TEXT
);

CREATE TABLE AmdProcessorList (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  model TEXT,
  family TEXT,
  line TEXT,
  platform TEXT,
  product_id_tray TEXT,
  product_id_boxed TEXT,
  product_id_mpk TEXT,
  launch_date TEXT,
  num_of_cpu_cores INTEGER,
  num_of_threads INTEGER,
  graphics_core_count TEXT,
  base_clock TEXT,
  max_boost_clock TEXT,
  all_core_boost_speed TEXT,
  l1_cache TEXT,
  l2_cache TEXT,
  l3_cache TEXT,
  ku_pricing TEXT,
  unlocked_for_overclocking TEXT,
  processor_technology_for_cpu_cores TEXT,
  cpu_socket TEXT,
  socket_count TEXT,
  pci_express_version TEXT,
  thermal_solution_pib TEXT,
  recommended_cooler TEXT,
  thermal_solution_mpk TEXT,
  default_tdp TEXT,
  amd_configurable_tdp_ctdp TEXT,
  max_operating_temperature_tjmax TEXT,
  os_support TEXT,
  system_memory_specification TEXT,
  system_memory_type TEXT,
  memory_channels INTEGER,
  per_socket_mem_bw TEXT,
  graphics_frequency TEXT,
  graphics_model TEXT,
  supported_technologies TEXT,
  workload_affinity TEXT,
  amd_ryzen_ai TEXT,
  fips_certification TEXT,
  fips_certification_links TEXT
);

CREATE TABLE IntelProcessorList (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  product_collection TEXT,
  vertical_segment TEXT,
  processor_number TEXT,
  lithography TEXT,
  use_conditions TEXT,
  recommended_customer_price TEXT,
  total_cores INTEGER,
  total_threads INTEGER,
  max_turbo_frequency TEXT,
  turbo_boost_technology_2_0_frequency TEXT,
  processor_base_frequency TEXT,
  cache TEXT,
  bus_speed TEXT,
  tdp TEXT,
  configurable_tdp_down_frequency TEXT,
  configurable_tdp_down TEXT,
  marketing_status TEXT,
  launch_date TEXT,
  embedded_options_available TEXT,
  included_items TEXT,
  description TEXT,
  functional_safety_documentation_available TEXT,
  servicing_status TEXT,
  end_of_servicing_updates_date TEXT,
  max_memory_size_dependent_on_memory_type TEXT,
  memory_types TEXT,
  max_num_of_memory_channels INTEGER,
  max_memory_bandwidth TEXT,
  ecc_memory_supported TEXT,
  processor_graphics TEXT,
  graphics_base_frequency TEXT,
  graphics_max_dynamic_frequency TEXT,
  graphics_video_max_memory TEXT,
  scalability TEXT,
  pci_express_revision TEXT,
  pci_express_configurations TEXT,
  max_num_of_pci_express_lanes TEXT,
  direct_media_interface_revision TEXT,
  max_num_of_dmi_lanes TEXT,
  intel_thunderbolt_4 TEXT,
  microprocessor_pcie_revision TEXT,
  chipset_pch_pcie_revision TEXT,
  sockets_supported TEXT,
  max_cpu_configuration TEXT,
  thermal_solution_specification TEXT,
  tjunction TEXT,
  package_size TEXT,
  intel_thermal_velocity_boost_temperature TEXT,
  operating_temperature_range TEXT,
  operating_temperature_maximum TEXT,
  operating_temperature_minimum TEXT,
  tcase TEXT,
  intel_optane_memory_supported TEXT,
  intel_turbo_boost_technology TEXT,
  intel_hyper_threading_technology TEXT,
  intel_aes_new_instructions TEXT,
  secure_key TEXT,
  intel_sg_extensions TEXT,
  intel_mp_extensions TEXT,
  intel_os_guard TEXT,
  intel_tx_technology TEXT,
  execute_disable_bit TEXT,
  intel_boot_guard TEXT,
  intel_stable_it_platform_program TEXT,
  intel_vt_technology TEXT,
  intel_vt_d TEXT,
  intel_vt_x_with_ept TEXT,
  intel_tdt TEXT,
  intel_amt TEXT,
  intel_ism TEXT,
  intel_one_click_recovery TEXT,
  intel_hardware_shield_eligibility TEXT,
  intel_cet TEXT,
  intel_total_memory_encryption_multi_key TEXT,
  intel_total_memory_encryption TEXT,
  mode_based_execute_control TEXT,
  intel_remote_platform_erase TEXT,
  intel_qas_acceleration TEXT,
  intel_vt_with_redirect_protection TEXT
);