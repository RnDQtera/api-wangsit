<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.003.001.004.Post Setting Role</name>
   <tag></tag>
   <elementGuidId>88a716d8-e0e5-45e5-8740-18dbefd63bb4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;QA Role\&quot;,\n  \&quot;description\&quot;: \&quot;QA Role\&quot;,\n  \&quot;permissions\&quot;: {\n    \&quot;administrasiProyek\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;read\&quot;: true,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: true\n    },\n    \&quot;timAndMember\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;read\&quot;: true,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: true\n    },\n    \&quot;memberAdmin\&quot;: {\n       \&quot;create\&quot;: true,\n      \&quot;read\&quot;: true,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;role\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;read\&quot;: true,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;poin\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;read\&quot;: true,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;proses\&quot;: {\n      \&quot;create\&quot;: true,\n      \&quot;read\&quot;: true,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;groupMember\&quot;: {\n       \&quot;create\&quot;: true,\n      \&quot;read\&quot;: true,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;sla\&quot;: {\n       \&quot;create\&quot;: true,\n      \&quot;read\&quot;: true,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false\n    }\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>58705321-1f21-4ecd-abd1-99ee88abaf6a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>55f81fd9-6725-410d-a2f4-efbaceb90a9a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${setting-role}${endpoint}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.setting_role</defaultValue>
      <description></description>
      <id>febfa1bd-a0ab-4582-af2e-499fbf137de0</id>
      <masked>false</masked>
      <name>setting-role</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>397a2051-f447-4ced-9a4e-c9ce5a5a74dc</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'roles'</defaultValue>
      <description></description>
      <id>e3f70a07-6466-42ff-9733-3599d2e4bbdf</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
