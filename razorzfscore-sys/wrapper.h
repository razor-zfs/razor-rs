#include <libzfs_core.h>
#include <libzfs.h>
//#include <sys/mnttab.h>
#include <libuutil.h>

extern zfs_handle_t *make_dataset_handle(libzfs_handle_t *, const char *);
extern char *zfs_strdup(libzfs_handle_t *, const char *);

typedef struct dmu_objset_stats {
	uint64_t dds_num_clones; /* number of clones of this */
	uint64_t dds_creation_txg;
	uint64_t dds_guid;
	dmu_objset_type_t dds_type;
	uint8_t dds_is_snapshot;
	uint8_t dds_inconsistent;
	uint8_t dds_redacted;
	char dds_origin[ZFS_MAX_DATASET_NAME_LEN];
} dmu_objset_stats_t;

struct libzfs_handle {
	int libzfs_error;
	int libzfs_fd;
	zpool_handle_t *libzfs_pool_handles;
	uu_avl_pool_t *libzfs_ns_avlpool;
	uu_avl_t *libzfs_ns_avl;
	uint64_t libzfs_ns_gen;
	int libzfs_desc_active;
	char libzfs_action[1024];
	char libzfs_desc[1024];
	int libzfs_printerr;
	boolean_t libzfs_mnttab_enable;
	/*
	 * We need a lock to handle the case where parallel mount
	 * threads are populating the mnttab cache simultaneously. The
	 * lock only protects the integrity of the avl tree, and does
	 * not protect the contents of the mnttab entries themselves.
	 */
	pthread_mutex_t libzfs_mnttab_cache_lock;
	avl_tree_t libzfs_mnttab_cache;
	int libzfs_pool_iter;
	boolean_t libzfs_prop_debug;
	regex_t libzfs_urire;
	uint64_t libzfs_max_nvlist;
	void *libfetch;
	char *libfetch_load_error;
};

struct zfs_handle {
	libzfs_handle_t *zfs_hdl;
	zpool_handle_t *zpool_hdl;
	char zfs_name[ZFS_MAX_DATASET_NAME_LEN];
	zfs_type_t zfs_type; /* type including snapshot */
	zfs_type_t zfs_head_type; /* type excluding snapshot */
	dmu_objset_stats_t zfs_dmustats;
	nvlist_t *zfs_props;
	nvlist_t *zfs_user_props;
	nvlist_t *zfs_recvd_props;
	boolean_t zfs_mntcheck;
	char *zfs_mntopts;
	uint8_t *zfs_props_table;
};