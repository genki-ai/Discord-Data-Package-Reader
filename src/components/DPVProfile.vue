<script setup lang="ts">
    const props = defineProps(["userData"])
</script>
<template>
    <div class="profileView">
        <h2>PFP</h2>
        <div class="pfp">
            <img style="max-width: 250px;" src="" alt="">
        </div>
        <table>
            <tr>
                <td>id:</td>
                <td>{{ props.userData.value.account.id }}</td>
            </tr>
            <tr>
                <td>username:</td>
                <td>{{ props.userData.value.account.username }}</td>
            </tr>
            <tr>
                <td>discriminator:</td>
                <td>#{{ props.userData.value.account.discriminator }}</td>
            </tr>
            <tr>
                <td>global name:</td>
                <td>{{ props.userData.value.account.global_name }}</td>
            </tr>
            <tr>
                <td>email:</td>
                <td>{{ props.userData.value.account.email }}</td>
            </tr>
            <tr>
                <td>is verified?:</td>
                <td>{{ props.userData.value.account.verified }}</td>
            </tr>
            <tr>
                <td>has mobile?:</td>
                <td>{{ props.userData.value.account.has_mobile }}</td>
            </tr>
            <tr>
                <td>phone number:</td>
                <td>{{ props.userData.value.account.phone }}</td>
            </tr>
            <tr>
                <td>ip:</td>
                <td>{{ props.userData.value.account.ip }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped>
    table {
        width: 100%;
    }
    td:nth-child(odd) {
        text-align: right;
        width: 20%;
        font-weight: bold;
    }
    table, th, td {
        border: 1px solid;
    }
    td {
        padding: 0.5em;
    }
    h2 {
        margin: 0;
    }
    .profileView {
        padding: 0.5em;
    }
    .pfp {
        border: 2px white solid;
        max-width: 200px;
        height: 200px;
    }
</style>